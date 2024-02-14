pub struct HttpRequest {
    pub method: String,
    pub uri: String,
    pub http_version: String,
}

impl HttpRequest {
    /// Parses the request line from a raw HTTP request string and returns an HttpRequest object.
    /// This function assumes the request string is correctly formatted.
    pub fn parse(request: &str) -> Option<HttpRequest> {
        let lines: Vec<&str> = request.lines().collect();
        if lines.is_empty() {
            return None;
        }

        let request_line: Vec<&str> = lines[0].split_whitespace().collect();
        if request_line.len() != 3 {
            return None;
        }

        Some(HttpRequest {
            method: request_line[0].to_string(),
            uri: request_line[1].to_string(),
            http_version: request_line[2].to_string(),
        })
    }
}
