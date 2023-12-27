use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServerError {
    message: String,
}

impl ServerError {
    pub fn new(message: &str) -> ServerError {
        ServerError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ServerError {}
