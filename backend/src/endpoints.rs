use log::info;
use poem_openapi::types::ToJSON;
use poem_openapi::{payload::*, OpenApi};

use crate::logic::Logic;

pub struct Api {
    pub lgc: Logic,
}

#[OpenApi]
impl Api {
    // healthchecks
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> Json<serde_json::Value> {
        info!("Healthcheck");
        let res = self.lgc.healthcheck().await;
        let payload = res.ok().to_json().expect("Something went wrong");
        Json(payload)
    }

    //* Healthcheck wont work with pool
    /* #[oai(path = "/db/healthcheck", method = "get")]
    async fn db_healthcheck(&self) -> Json<serde_json::Value> {
        let res = self.lgc.db_healthcheck().await;
        let payload = res.ok().to_json().expect("Something went wrong");
        Json(payload)
    } */

    // graph API endpoints
    #[oai(path = "/graph/me", method = "get")]
    async fn getting_myself(&self) -> Json<serde_json::Value> {
        let res = self.lgc.graph_get_self().await;
        let payload = res.ok().to_json().expect("Something went wrong");
        Json(payload)
    }
}
