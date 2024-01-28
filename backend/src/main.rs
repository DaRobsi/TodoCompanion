mod endpoints;
mod graph_communicator;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() {
    let api_service =
        OpenApiService::new(endpoints::Api, "Hello World", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/docs", ui);

    // prb better to use envvar for port in TodoCompanion
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
