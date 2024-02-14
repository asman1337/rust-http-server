pub struct HttpResponse {
    status_code: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpResponse {
    /// Creates a new HttpResponse object.
    pub fn new(status_code: &str, headers: Vec<(String, String)>, body: &str) -> HttpResponse {
        HttpResponse {
            status_code: status_code.to_string(),
            headers,
            body: body.to_string(),
        }
    }

    /// Converts the HttpResponse object into a raw HTTP response string.
    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status_code);
        for (header, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", header, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
