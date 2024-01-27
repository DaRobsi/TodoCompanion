use dotenv::dotenv;

pub async fn get_myself() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    const URL: &str = "https://graph.microsoft.com/v1.0/me";
    let token = std::env::var("GRAPH_TOKEN").unwrap().to_string();

    let client = reqwest::Client::new();
    let resp = client
        .get(URL)
        .bearer_auth(token)
        .send()
        .await?
        .json::<()>()
        .await?;

    println!("{:#?}", resp);

    Ok(())
}
