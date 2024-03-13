use dotenv::dotenv;
use graph_rs_sdk::*;

// Return a GraphResult which on success gives a JSON
pub async fn get_self() -> GraphResult<serde_json::value::Value> {
    dotenv().ok();
    // use token provided through the .env 
    let token = std::env::var("GRAPH_TOKEN").unwrap().to_string();

    let client = Graph::new(&token);
    let response = client.me().get_user().send().await?;

    let body: serde_json::Value = response.json().await?;

    Ok(body)
}
