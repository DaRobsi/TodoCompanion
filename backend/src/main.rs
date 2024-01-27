mod endpoints;
mod graph_communicator;

//use poem::{listener::TcpListener, Route, Server};
//use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    graph_communicator::get_myself().await?;
    Ok(())
}
