mod endpoints;
mod graph_communicator;

//use poem::{listener::TcpListener, Route, Server};
//use poem_openapi::OpenApiService;

#[tokio::main]
async fn main(){
    let result = graph_communicator::get_self().await.unwrap();
    println!("{}", result);
}
