mod endpoints;
mod graph_communicator;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use log::info;

#[tokio::main]
async fn main() {
    // initialize proper logging 
    env_logger::init();
    // create API service
    let api_service =
        OpenApiService::new(endpoints::Api, "Todo Companion", "1.0").server("http://localhost:3000/api");
    // create docs
    let ui = api_service.swagger_ui();
    // bind both to app
    let app = Route::new().nest("/api", api_service).nest("/docs", ui);

    // prb better to use envvar for port in TodoCompanion
    info!("Starting API service...");
    // run the app
    let _ = Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await;
}
