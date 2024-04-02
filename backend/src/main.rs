mod db_handler;
mod endpoints;
mod graph_communicator;
mod logic;

use dotenv::dotenv;
use log::info;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize proper logging
    env_logger::init();
    // initialize dotenv
    dotenv().ok();
    // initialize database handler
    let dbh = db_handler::DBHandler::new().await?;
    let graph_comm = graph_communicator::Graph_Communicator::new().await?;
    let lgc = logic::Logic {db_conn: dbh, graph_conn: graph_comm};
    // create API service
    let api_service = OpenApiService::new(endpoints::Api { lgc: lgc }, "Todo Companion", "1.0")
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
