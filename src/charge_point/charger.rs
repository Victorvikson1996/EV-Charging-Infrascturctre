use serde_json::json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};
use crate::ocpp::{OcppMessage, BootNotification, StatusNotification, StartTransaction, StopTransaction, Heartbeat, MeterValues};


pub  struct Charger {
    id: String,
    power_rating: f64,
    energy_delivered: f64,
    is_charging: bool,
    transaction_id: String,
}

impl Charger {
    //Boot
    pub fn new(id: &str, power_rating: f64) -> Self {
        Charger {
            id: id.to_string(),
            power_rating,
            energy_delivered,
            is_charging: false,
            transaction_id: None
        }
    }


    pub async fn run (&mut self, send: &mut impl FnMut(OcppMessage) -> ()) {
        send(OcppMessage::new_call("BootNotification", BootNotification {
            charge_point_id: self.id.clone(),
            vendor: "DynoCharge".to_string(),
            model: "DynoCharge".to_string(),
        }));


        send(OcppMessage::new_call("StatusNotification", StatusNotification {
            connector_id: 1,
            status: "Available".to_string(),
        }));

        tokio::spawn({
            let mut send = send.clone();
            async move {
                loop {
                    sleep(Duration:: from_secs(60)).await;
                    send(OcppMessage::new_call("Heartbeat", Heartbeat {
                        current_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                        interval: 60,
                    }));
                }
            }
        })

    }
}
