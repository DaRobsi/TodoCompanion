mod endpoints;
mod graph_communicator;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    let api_service =
        OpenApiService::new(endpoints::Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/docs", ui);

    // prb better to use envvar for port in TodoCompanion
    info!("Starting API service...");
    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
