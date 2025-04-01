use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use sqlx::SqlitePool;
use crate::ocpp::messages::OcppMessage;

pub struct CMS {
    rate_per_kwh: f64,
    db: SqlitePool,
}

impl CMS {
    pub fn new(rate_per_kwh: f64, db: SqlitePool) -> Self {
        CMS { rate_per_kwh, db }
    }

    pub async fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.expect("Failed to bind");
        println!("CMS listening on {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream).await.expect("Failed to accept");
            let (mut write, mut read) = ws_stream.split();
            let db = self.db.clone();
            let rate = self.rate_per_kwh;

            tokio::spawn(async move {
                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        let request: OcppMessage = serde_json::from_str(&text).unwrap();
                        match request {
                            OcppMessage::Call { message_id, action, payload } => {
                                let response = match action.as_str() {
                                    "BootNotification" => {
                                        println!("Charger {} booted", payload["charge_point_id"]);
                                        OcppMessage::new_result(message_id, json!({"status": "Accepted"}))
                                    }
                                    "StatusNotification" => {
                                        println!("Status update: {:?}", payload);
                                        OcppMessage::new_result(message_id, json!({"status": "Accepted"}))
                                    }
                                    "StartTransaction" => {
                                        let tx_id = uuid::Uuid::new_v4().to_string();
                                        println!("Transaction started: {}", tx_id);
                                        OcppMessage::new_result(message_id, json!({"transaction_id": tx_id, "status": "Accepted"}))
                                    }
                                    "StopTransaction" => {
                                        let energy = payload["energy"].as_f64().unwrap();
                                        let cost = energy * rate;
                                        let tx_id = payload["transaction_id"].as_str().unwrap().to_string();
                                        sqlx::query("INSERT INTO transactions (id, charger_id, energy, cost, timestamp) VALUES (?, ?, ?, ?, ?)")
                                            .bind(&tx_id)
                                            .bind("CP_001")
                                            .bind(energy)
                                            .bind(cost)
                                            .bind(payload["timestamp"].as_str().unwrap())
                                            .execute(&db)
                                            .await
                                            .unwrap();
                                        println!("Transaction stopped. Energy: {} kWh, Cost: ${}", energy, cost);
                                        OcppMessage::new_result(message_id, json!({"status": "Accepted"}))
                                    }
                                    "Heartbeat" => {
                                        OcppMessage::new_result(message_id, json!({"current_time": chrono::Utc::now().to_rfc3339()}))
                                    }
                                    "MeterValues" => {
                                        println!("Meter value: {:?}", payload);
                                        OcppMessage::new_result(message_id, json!({"status": "Accepted"}))
                                    }
                                    _ => unreachable!(),
                                };
                                write.send(Message::Text(serde_json::to_string(&response).unwrap().into())).await.unwrap();
                            }
                            _ => println!("Unhandled message: {:?}", text),
                        }
                    }
                }
            });
        }
    }
}


