use log::info;
use poem_openapi::types::ToJSON;
use poem_openapi::{payload::{self, *}, OpenApi};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use serde_json::Error;
use sqlx::postgres::PgConnection;
use graph_rs_sdk::GraphResult;

use crate::db_handler::DBHandler;
use crate::graph_communicator::Graph_Communicator;

#[derive(Serialize, Deserialize)]
struct Message {
    status: u16,
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct GraphResponse {
    status: u16,
    resp: serde_json::Value
}

pub struct Logic {
    pub db_conn: DBHandler,
    pub graph_conn: Graph_Communicator
}

impl Logic {
    pub async fn healthcheck(&self) -> Result<String, serde_json::Error> {
        info!("Healthcheck");
        let payload = to_string(&Message {
            status: 200,
            msg: "This works!".to_string(),
        });
        return payload
    }

    pub async fn db_healthcheck(&mut self) -> Result<String, serde_json::Error> {
        info!("Checking database connectivity...");
        let res = self.db_conn.ping_database().await;
        if res.is_ok() {
            let payload = to_string(&Message {
                status: 200,
                msg: "Database is alive!".to_string(),
            });
            return payload
        } else {
            let payload = to_string(&Message {
                status: 500,
                msg: "Could not reach database".to_string(),
            });
            return payload
        }
    }

    pub async fn graph_get_self(&self) -> Result<String, serde_json::Error> {
        let res = self.graph_conn.get_self().await;
        if res.is_ok() {
            let payload = to_string(&GraphResponse {
                status: 200,
                resp: res.unwrap()
            });
            return payload
        } else {
            let payload = to_string(&GraphResponse {
                status: 500,
                resp: res.unwrap()
            });
            return payload
        }
    }


}
