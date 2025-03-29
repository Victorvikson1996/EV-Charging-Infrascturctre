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
        });

        sleep(Duration::from_secs(10)).await;
        self.is_charging = true;
        let timestamp = Self::get_timestamp();
        send(OcppMessage::new_call("StartTransaction", StartTransaction {
            connector_id: 1,
            id_tag: "USER123".to_string(),
            timestamp: timestamp.clone(),
        }));


        // Send MeterValues every 10 seconds
        for _ in 0..6 { // 1 minute total
            sleep(Duration::from_secs(10)).await;
            self.energy_delivered += self.power_rating * (10.0 / 3600.0); // 10s in hours
            if let Some(tx_id) = &self.transaction_id {
                send(OcppMessage::new_call("MeterValues", MeterValues {
                    connector_id: 1,
                    transaction_id: tx_id.clone(),
                    energy: self.energy_delivered,
                    timestamp: Self::timestamp(),
                }));
            }
        }

        self.is_charging = false;
        send(OcppMessage::new_call("StatusNotification", StatusNotification {
            connector_id: 1,
            status: "Available".to_string(),
        }));
        if let Some(tx_id) = self.transaction_id.take() {
            send(OcppMessage::new_call("StopTransaction", StopTransaction {
                transaction_id: tx_id,
                energy: self.energy_delivered,
                timestamp: Self::timestamp(),
            }));
        }
    }


    pub fn set_transaction_id(&mut self, tx_id: String) {
        self.transaction_id = Some(tx_id);
    }

    fn timestamp() -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        chrono::DateTime::<chrono::Utc>::from_timestamp(now as i64, 0)
            .unwrap()
            .to_rfc3339()
    }
}
