use poem_openapi::{payload::*, OpenApi};

use crate::graph_communicator;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> PlainText<&'static str> {
        PlainText("This works!")
    }

    #[oai(path = "/graph/me", method = "get")]
    async fn getting_myself(&self) -> Json<serde_json::value::Value> {
        Json(
            graph_communicator::get_self()
                .await
                .expect("Something went wrong"),
        )
    }
}
