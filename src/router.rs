use std::collections::HashMap;

use crate::request::HttpRequest;
use crate::response::HttpResponse;

/// Type alias for handler functions.
/// These functions take an HttpRequest and return an HttpResponse.
pub type Handler = fn(HttpRequest) -> HttpResponse;

/// The Router struct maps paths to handler functions.
pub struct Router {
    routes: HashMap<String, Handler>,
}

impl Router {
    /// Creates a new Router instance.
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }

    /// Adds a route to the Router.
    pub fn add_route(&mut self, path: &str, handler: Handler) {
        self.routes.insert(path.to_string(), handler);
    }

    /// Handles an incoming request by finding and invoking the appropriate handler.
    /// Returns a default response if no handler is found for the request path.
    pub async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        if let Some(handler) = self.routes.get(&request.uri) {
            handler(request)
        } else {
            // No handler found for the request path
            HttpResponse::new(
                "404 Not Found",
                vec![("Content-Type".to_string(), "text/plain".to_string())],
                "404 Not Found",
            )
        }
    }
}
