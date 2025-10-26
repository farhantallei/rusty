#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub body: String,
}

impl Request {
    pub fn new(path: String, body: String) -> Self {
        Self { path, body }
    }
}
