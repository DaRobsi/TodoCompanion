use std::path::PathBuf;

use poem_openapi::types::ToJSON;
use poem_openapi::{payload::*, OpenApi};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{postgres, Connection};
use log::info;

use crate::db_handler::DBHandler;
use crate::graph_communicator;

#[derive(Serialize, Deserialize)]
struct Message {
    status: u16,
    msg: String
}

pub struct Api {
    pub db_conn: DBHandler
}

#[OpenApi]
impl Api {
    // healthchecks
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> PlainText<&'static str> {
        info!("Healthcheck");
        PlainText("This works!")
    }

    #[oai (path = "/db/healthcheck", method = "get")]
    async fn db_healthcheck(&self) -> Json<serde_json::Value> {
        let res = self.db_conn.ping_database().await;
        if res.is_ok() {
            let payload = to_string(&Message {status: 200, msg: "Database is alive!".to_string()});
            let final_payload = payload.ok().to_json().expect("Something went wrong");
            Json(final_payload)
        } else {
            let payload = to_string(&Message {status: 500, msg: "Could not reach database".to_string()});
            let final_payload = payload.ok().to_json().expect("Something went wrong");
            Json(final_payload)
        }
    }

    // graph API endpoints
    #[oai(path = "/graph/me", method = "get")]
    async fn getting_myself(&self) -> Json<serde_json::Value> {
        info!("Fetching own account...");
        Json(
            graph_communicator::get_self()
                .await
                .expect("Something went wrong")
        )
    }
}
