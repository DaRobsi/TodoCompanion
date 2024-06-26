use serde::{Deserialize, Serialize};
use sqlx::Type;

// API models
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub status: u16,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbResponse {
    pub status: u16,
    pub resp: String,
}

#[derive(Serialize, Deserialize)]
pub struct GraphResponse {
    pub status: u16,
    pub resp: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct SetOfNotes {
    pub notes: Vec<String>
}

// DB models
#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub time_added: String,
    pub details: NoteDetails,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct NoteDetails {
    pub tags: Vec<String>,
}
