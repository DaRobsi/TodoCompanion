use log::info;
use serde_json::to_string;
use sqlx::Row;

use crate::db_handler::DBHandler;
use crate::graph_communicator::GraphCommunicator;
use crate::models::*;

pub struct Logic {
    pub db_conn: DBHandler,
    pub graph_conn: GraphCommunicator,
}

impl Logic {
    pub async fn healthcheck(&self) -> Result<String, serde_json::Error> {
        info!("Healthcheck");
        let payload = to_string(&Message {
            status: 200,
            msg: "This works!".to_string(),
        });
        return payload;
    }

    pub async fn graph_get_self(&self) -> Result<String, serde_json::Error> {
        let res = self.graph_conn.get_self().await;
        if res.is_ok() {
            let payload = to_string(&GraphResponse {
                status: 200,
                resp: res.unwrap(),
            });
            return payload;
        } else {
            let payload = to_string(&GraphResponse {
                status: 500,
                resp: res.unwrap(),
            });
            return payload;
        }
    }

    pub async fn db_get_all_notes(&self) -> Result<String, serde_json::Error> {
        let res = self.db_conn.get_all_notes().await;
        if res.is_ok() {
            let res = res.ok().unwrap();
            let recieved_note = to_string(&Note {
                id: res.get("id"),
                title: res.get("title"),
                content: res.get("content"),
                time_added: res.get("time_added"),
                details: res.get("details"),
            });
            let payload = to_string(&DbResponse {
                status: 200,
                resp: recieved_note.unwrap(),
            })
            .unwrap();
            Ok(payload)
        } else {
            let payload = to_string(&Message {
                status: 500,
                msg: res.err().unwrap().to_string(),
            })
            .unwrap_err();
            Err(payload)
        }
    }
}
