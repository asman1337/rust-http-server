use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::request::HttpRequest;
use crate::response::HttpResponse;

/// Type alias for handler functions.
/// These functions take an HttpRequest and return an HttpResponse.
pub type Handler = fn(HttpRequest) -> HttpResponse;

/// Special handler type for static file serving, allowing for more flexible handling
pub type StaticHandler = Arc<dyn Fn(&str) -> Pin<Box<dyn Future<Output=HttpResponse> + Send>> + Send + Sync>;

/// The Router struct maps paths to handler functions.
pub struct Router {
    routes: HashMap<String, Handler>,
    static_routes: HashMap<String, StaticHandler>,
}

impl Router {
    /// Creates a new Router instance.
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
            static_routes: HashMap::new(),
        }
    }

    /// Adds a route to the Router.
    pub fn add_route(&mut self, path: &str, handler: Handler) {
        self.routes.insert(path.to_string(), handler);
    }

    /// Adds a static route to the Router for serving files from a directory.
    pub fn add_static_route(&mut self, prefix: &str, handler: StaticHandler) {
        self.static_routes.insert(prefix.to_string(), handler);
    }

    /// Handles an incoming request by finding and invoking the appropriate handler.
    /// Tries static routes if no exact match is found.
    pub async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        // First, attempt to match a regular route
        if let Some(handler) = self.routes.get(&request.uri) {
            return handler(request);
        }

        // If no regular route matches, attempt to match static routes
        for (prefix, handler) in &self.static_routes {
            if request.uri.starts_with(prefix) {
                let static_path = &request.uri[prefix.len()..];
                return (handler)(static_path).await;
            }
        }

        // No handler found for the request path
        HttpResponse::new(
            "404 Not Found",
            vec![("Content-Type".to_string(), "text/plain".to_string())],
            "404 Not Found",
        )
    }
}
