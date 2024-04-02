use graph_rs_sdk::*;
use std::error::Error;

pub struct GraphCommunicator {
    graph_token: String
}

impl GraphCommunicator {
    pub async fn new() -> Result<GraphCommunicator, Box<dyn Error>> {
        let token = std::env::var("GRAPH_TOKEN").unwrap().to_string();

        Ok(GraphCommunicator { graph_token: token})
    }

    // Return a GraphResult which on success gives a JSON
    pub async fn get_self(&self) -> GraphResult<serde_json::Value> {
        let client = Graph::new(self.graph_token.as_str());
        let response = client.me().get_user().send().await?;

        let body: serde_json::Value = response.json().await?;

        Ok(body)
    }
}


