use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::{fs, thread};

use self::errors::ServerError;
pub mod errors;

#[derive(Debug, Clone)]
pub struct Server {
    address: String,
    port: u16,
    clipboard_file: String,
    password: Option<String>,
}

impl Server {
    pub fn new(
        address: &str,
        port: u16,
        clipboard_file: &str,
        pwd: Option<String>,
    ) -> Result<Self, ServerError> {
        let full_address = format!("{}:{}", address, port);
        if full_address.to_socket_addrs().is_ok() {
            Ok(Server {
                address: address.to_string(),
                port,
                clipboard_file: clipboard_file.to_string(),
                password: pwd,
            })
        } else {
            Err(ServerError::new("Invalid address"))
        }
    }

    fn listen(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.address, self.port))
    }

    fn handle_client(&self, mut stream: TcpStream) {
        // Optional: Authenticate client with password
        if let Some(ref password) = self.password {
            let mut password_buffer = [0; 1024]; // Adjust buffer size as needed
            match stream.read(&mut password_buffer) {
                Ok(size) => {
                    let received_password = String::from_utf8_lossy(&password_buffer[..size])
                        .trim()
                        .to_string();
                    if received_password != *password {
                        log::error!("Authentication failed");
                        return; // Disconnect if password doesn't match
                    }
                }
                Err(e) => {
                    log::error!("Failed to read password: {}", e);
                    return;
                }
            }
        }

        // Clear the clipboard file initially
        fs::write(&self.clipboard_file, "").expect("Failed to clear clipboard file");

        // Main loop to handle clipboard synchronization
        loop {
            let current_clip =
                fs::read_to_string(&self.clipboard_file).unwrap_or_else(|_| String::new());

            if !current_clip.is_empty() {
                log::debug!("Sending clipboard contents to client: {}", current_clip);
                if let Err(e) = stream.write_all(current_clip.as_bytes()) {
                    log::error!("Failed to send clipboard contents to client: {}", e);
                    thread::sleep(Duration::from_secs(1));
                }
                // Clear the clipboard file after sending contents
                fs::write(&self.clipboard_file, "").expect("Failed to clear clipboard file");
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = self.listen()?;

        log::info!("Server listening on {}", listener.local_addr()?);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    log::debug!("New connection: {}", stream.peer_addr()?);
                    let server_clone = self.clone();
                    thread::spawn(move || server_clone.handle_client(stream));
                }
                Err(e) => {
                    log::error!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
