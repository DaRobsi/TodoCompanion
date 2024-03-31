use poem_openapi::{payload::*, OpenApi};
use sqlx::{postgres, Connection};
use log::info;

use crate::db_handler::DBHandler;
use crate::graph_communicator;

pub struct Api;

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
        info!("Checking Database...");
        let res = postgres::PgConnection::ping(&mut DBHandler::db_connection);
        Json(res)
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
