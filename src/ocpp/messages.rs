

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "message_type")]
pub enum OcppMessage {
    #[serde(rename = "CALL")]
    Call {
        message_id: String,
        action: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "CALLRESULT")]
    CallResult {
        message_id: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "CALLERROR")]
    CallError {
        message_id: String,
        error_code: String,
        error_description: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootNotification {
    pub charge_point_id: String,
    pub vendor: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusNotification {
    pub connector_id: u32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartTransaction {
    pub connector_id: u32,
    pub id_tag: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopTransaction {
    pub transaction_id: String,
    pub energy: f64,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heartbeat {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeterValues {
    pub connector_id: u32,
    pub transaction_id: String,
    pub energy: f64,
    pub timestamp: String,
}

impl OcppMessage {
    pub fn new_call(action: &str, payload: impl Serialize) -> Self {
        OcppMessage::Call {
            message_id: Uuid::new_v4().to_string(),
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
}

#[allow(unused_imports)] // Suppress warning for now
pub mod prelude {
    pub use super::{
        BootNotification, Heartbeat, MeterValues, OcppMessage, StartTransaction,
        StatusNotification, StopTransaction,
    };
}
