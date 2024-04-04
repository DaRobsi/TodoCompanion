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
        let res = self.lgc.healthcheck().await;
        let payload = res.ok().to_json().expect("Something went wrong");
        Json(payload)
    }

    // graph API endpoints
    #[oai(path = "/graph/me", method = "get")]
    async fn getting_myself(&self) -> Json<serde_json::Value> {
        let res = self.lgc.graph_get_self().await;
        let payload = res.ok().to_json().expect("Something went wrong");
        Json(payload)
    }

    // db endpoints
    #[oai(path = "/db/notes/all", method = "get")]
    async fn get_all_notes(&self) -> Json<serde_json::Value> {
        let res = self.lgc.db_get_all_notes().await;
        let payload = res.ok().to_json().expect("Something went wrong");
        Json(payload)
    }
}
