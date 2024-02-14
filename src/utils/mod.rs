use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::response::HttpResponse;

pub async fn serve_static_file(path: &str) -> HttpResponse {
    let sanitized_path = path.replace("..", "");
    let file_path = format!("./static{}", sanitized_path);
    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = vec![];
            if let Ok(_) = file.read_to_end(&mut contents).await {
                let content_type = match std::path::Path::new(&file_path).extension().and_then(std::ffi::OsStr::to_str) {
                    Some("html") => "text/html",
                    Some("css") => "text/css",
                    Some("js") => "application/javascript",
                    _ => "application/octet-stream",
                };

                HttpResponse::new(
                    "200 OK",
                    vec![("Content-Type".to_string(), content_type.to_string())],
                    &String::from_utf8_lossy(&contents),
                )
            } else {
                HttpResponse::new("500 Internal Server Error", vec![], "Failed to read file")
            }
        }
        Err(_) => HttpResponse::new("404 Not Found", vec![], "File not found"),
    }
}
