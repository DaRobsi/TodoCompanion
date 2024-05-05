mod db_handler;
mod endpoints;
mod logic;
mod models;

use dotenv::dotenv;
use log::{info, warn};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize proper logging
    env_logger::init();
    // initialize dotenv
    dotenv().ok();
    // initialize logic with DBHandler and Graph_Communicator
    info!("Establishing db connection...");
    let dbh = db_handler::DBHandler::new().await?;
    info!("Initializing Logic component with {:#?}", &dbh);
    let lgc = logic::Logic {
        db_conn: dbh
    };
    // create API service
    let api_service = OpenApiService::new(endpoints::Api { lgc }, "Todo Companion", "1.0")
        .server("http://localhost:3000/api");
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
