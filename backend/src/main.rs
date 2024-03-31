mod endpoints;
mod graph_communicator;
mod db_handler;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use log::info;
use dotenv::dotenv;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize proper logging 
    env_logger::init();
    // initialize dotenv
    dotenv().ok();
    let dbh = db_handler::DBHandler::new();
    // create API service
    let api_service =
        OpenApiService::new(endpoints::Api, "Todo Companion", "1.0").server("http://localhost:3000/api");
    // create docs
    let ui = api_service.swagger_ui();
    // bind both to app
    let app = Route::new().nest("/api", api_service).nest("/docs", ui);
    info!("Starting API service...");
    // run the app
    let _ = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await;

    Ok(())
}
