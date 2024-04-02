use log::info;
use serde_json::to_string;

use crate::db_handler::DBHandler;
use crate::graph_communicator::GraphCommunicator;
use crate::models::*;

pub struct Logic {
    pub db_conn: DBHandler,
    pub graph_conn: GraphCommunicator
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

    //* Healthcheck wont work with pool
    /* pub async fn db_healthcheck(&self) -> Result<String, serde_json::Error> {
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
    } */

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
