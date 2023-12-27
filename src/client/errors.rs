#[derive(Debug)]
pub struct ClientError {
    message: String,
}

impl ClientError {
    pub fn new(message: &str) -> ClientError {
        ClientError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Client error: {}", self.message)
    }
}

impl std::error::Error for ClientError {}
