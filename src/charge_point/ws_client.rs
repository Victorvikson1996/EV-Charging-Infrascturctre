use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::{SinkExt, StreamExt};
use crate::charge_point::charger::Charger;

pub async fn connect(id: &str, power_rating: f64, ws_url: &str) {
    let (ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");
    println!("Charge Point {} connected to CMS", id);
    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<OcppMessage>(100);

    // Send messages from charger
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let text = serde_json::to_string(&msg).unwrap();
            write.send(Message::Text(text)).await.unwrap();
        }
    });

    // Charger logic
    let mut charger = Charger::new(id, power_rating);
    tokio::spawn(async move {
        charger.run(&mut |msg| { tx.try_send(msg).unwrap(); }).await;
    });

    // Handle CMS responses
    while let Some(msg) = read.next().await {
        if let Ok(Message::Text(text)) = msg {
            let response: OcppMessage = serde_json::from_str(&text).unwrap();
            match response {
                OcppMessage::CallResult { message_id: _, payload } => {
                    if let Some(tx_id) = payload.get("transaction_id").and_then(|v| v.as_str()) {
                        charger.set_transaction_id(tx_id.to_string());
                    }
                }
                _ => println!("Unhandled message: {:?}", text),
            }
        }
    }
}