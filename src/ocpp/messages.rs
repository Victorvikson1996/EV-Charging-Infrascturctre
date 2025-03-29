use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "message_type")]

pub enum OcppMessge {
    #[serde(rename = "CALL")]
    Call {
        message_id: String,
        action: String,
        payload: serde_json::Value,
    },

    #[serde(rename = "CALL_RESULT")]
    CallResult {
        message_id: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "CALL_ERROR")]
    CallError {
        message_id: String,
        error_code: String,
        error_description: String,
        error_details: Option<serde_json::Value>,
    },
    // #[serde(rename = "COST")]
    // Cost {
    //     message_id: String,
    //     payload: serde_json::Value,
    // },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BootNotification {
    pub charge_point_id: String,
    pub vendor: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusNotification {
    pub connector_id: u32,
    pub status: String,
    // pub error_code: Option<String>,
    // pub info: Option<String>,
    // pub status_notification: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartTransaction {
    pub connector_id: u32,
    pub id_tag: String,
    pub timestamp: String, // pub meter_start: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopTransaction {
    pub transaction_id: String,
    pub energy: f64, //kwh
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat {
    pub current_time: String,
    pub interval: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeterValues {
    pub connector_id: u32,
    pub transaction_id: String,
    pub energy: f64, //kwh
    pub timestamp: String,
}

impl OcppMessage {
    pub fn new_call(action: &str, payload: impl Serialize) -> Self {
        OcppMessage::Call {
            message_id: Uuid::new_V4().to_string(),
            action: action.to_string(),
            payload: serde_json::to_value(payload).unwrap(),
        }
    }

    pub fn new_result(message_id: String, payload: impl Serialize) -> Self {
        OcppMessage::CallResult {
            message_id,
            payload: serde_json::to_value(payload).unwrap(),
        }
    }

    pub mod prelude {
        pub use super::*;
    }
}
