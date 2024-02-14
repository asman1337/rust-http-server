use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::request::HttpRequest;
use crate::response::HttpResponse;
use crate::router::Router;

pub async fn handle_connection(mut socket: TcpStream, router: &Router) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(_) => {
            let request_str = match std::str::from_utf8(&buffer) {
                Ok(v) => v,
                Err(e) => {
                    println!("Invalid UTF-8 sequence: {}", e);
                    return;
                },
            };

            if let Some(request) = HttpRequest::parse(request_str) {
                println!("Parsed Request: {} {} {}", request.method, request.uri, request.http_version);

                // Use the router to handle the request and generate a response
                let response = router.handle_request(request).await;

                // Convert the response to a string and write it back to the socket
                if let Err(e) = socket.write_all(response.to_string().as_bytes()).await {
                    println!("Failed to write response: {}", e);
                }
            } else {
                println!("Failed to parse request.");

                // If the request cannot be parsed, send back a 400 Bad Request response
                let response = HttpResponse::new(
                    "400 Bad Request",
                    vec![("Content-Type".to_string(), "text/plain".to_string())],
                    "Bad Request",
                );
                if let Err(e) = socket.write_all(response.to_string().as_bytes()).await {
                    println!("Failed to write response: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to read from socket: {}", e);

            // Optionally, we can handle the error more gracefully
        }
    }
}
