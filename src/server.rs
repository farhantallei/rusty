use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use super::{Request, Response};

type Handler = fn(&Request) -> Response;

pub struct Rusty {
    routes: HashMap<(String, String), Handler>,
}

impl Rusty {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.routes
            .insert(("GET".to_string(), path.to_string()), handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        self.routes
            .insert(("POST".to_string(), path.to_string()), handler);
    }

    pub fn put(&mut self, path: &str, handler: Handler) {
        self.routes
            .insert(("PUT".to_string(), path.to_string()), handler);
    }

    pub fn delete(&mut self, path: &str, handler: Handler) {
        self.routes
            .insert(("DELETE".to_string(), path.to_string()), handler);
    }

    pub fn listen(&self, port: u16) {
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
        println!("🚀 Rusty running at http://127.0.0.1:{port}");

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                self.handle_connection(stream);
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        if stream.read(&mut buffer).is_err() {
            return;
        }

        let request_str = String::from_utf8_lossy(&buffer);
        let mut lines = request_str.lines();
        let first_line = lines.next().unwrap_or("");
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        let method = parts.get(0).unwrap_or(&"GET").to_string();
        let path = parts.get(1).unwrap_or(&"/").to_string();

        let body = request_str
            .split("\r\n\r\n")
            .nth(1)
            .unwrap_or("")
            .to_string();

        let req = Request::new(path.clone(), body);

        let res = if let Some(handler) = self.routes.get(&(method.clone(), path)) {
            handler(&req)
        } else {
            Response::new(404, "Not Found")
        };

        let response_text = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            res.status,
            res.body.len(),
            res.body
        );

        let _ = stream.write_all(response_text.as_bytes());
        let _ = stream.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Request, Response};

    fn handler(_req: &Request) -> Response {
        Response::new(200, "Hello, Rusty 🚀!")
    }

    #[test]
    fn test_register_get_route() {
        let mut rusty = Rusty::new();
        rusty.get("/hello", handler);
        assert!(
            rusty
                .routes
                .contains_key(&("GET".to_string(), "/hello".to_string()))
        );
    }

    #[test]
    fn test_register_post_route() {
        let mut rusty = Rusty::new();
        rusty.post("/submit", handler);
        assert!(
            rusty
                .routes
                .contains_key(&("POST".to_string(), "/submit".to_string()))
        );
    }

    #[test]
    fn test_register_put_route() {
        let mut rusty = Rusty::new();
        rusty.put("/update", handler);
        assert!(
            rusty
                .routes
                .contains_key(&("PUT".to_string(), "/update".to_string()))
        );
    }

    #[test]
    fn test_register_delete_route() {
        let mut rusty = Rusty::new();
        rusty.delete("/remove", handler);
        assert!(
            rusty
                .routes
                .contains_key(&("DELETE".to_string(), "/remove".to_string()))
        );
    }

    #[test]
    fn test_request_with_body() {
        let req = Request::new("/submit".to_string(), "name=rusty".to_string());
        assert_eq!(req.path, "/submit");
        assert_eq!(req.body, "name=rusty");
    }
}
