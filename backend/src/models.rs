use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub status: u16,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct GraphResponse {
    pub status: u16,
    pub resp: serde_json::Value
}