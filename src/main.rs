use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::Router;

mod server;
mod middleware;
mod utils;
mod request;
mod response;
mod router;


// Define a simple handler for the root path
fn root_handler(_req: HttpRequest) -> HttpResponse {
    HttpResponse::new(
        "200 OK",
        vec![("Content-Type".to_string(), "text/plain".to_string())],
        "Welcome to our Rust HTTP Server!",
    )
}

// Define another handler for a `/hello` path
fn hello_handler(_req: HttpRequest) -> HttpResponse {
    HttpResponse::new(
        "200 OK",
        vec![("Content-Type".to_string(), "text/plain".to_string())],
        "Hello, world!",
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Await the result of TcpListener::bind
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    // Initialize the router and register routes
    let mut router = Router::new();
    router.add_route("/", root_handler);
    router.add_route("/hello", hello_handler);

    let router = Arc::new(router);

    loop {
        let (socket, _) = listener.accept().await?;
        let router_clone = router.clone();

        tokio::spawn(async move {
            server::handle_connection(socket, &router_clone).await;
        });
    }
}


