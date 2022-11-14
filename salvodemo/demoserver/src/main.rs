use salvo::prelude::*;

mod pages;

use pages::new_router;

#[tokio::main]
async fn main() {
    let router = new_router();
    let tcp = TcpListener::bind("127.0.0.1:7878");
    let server = Server::new(tcp);
    server.serve(router).await;
}