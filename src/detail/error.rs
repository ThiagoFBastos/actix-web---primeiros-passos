use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    pub message: String
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string()
        }
    }
}