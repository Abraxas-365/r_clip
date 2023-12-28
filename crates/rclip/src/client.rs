use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread;
use std::time::Duration;

use crate::clipboard_listener::ClipboardListener;

use self::errors::ClientError;
pub mod errors;

#[derive(Debug, Clone)]
pub struct Client {
    server_address: String,
    port: u16,
    password: Option<String>,
}

impl Client {
    pub fn new(server_address: &str, port: u16, pwd: Option<String>) -> Result<Self, ClientError> {
        let full_address = format!("{}:{}", server_address, port);
        if full_address.to_socket_addrs().is_ok() {
            Ok(Client {
                server_address: server_address.to_string(),
                port,
                password: pwd,
            })
        } else {
            Err(ClientError::new("Invalid server address"))
        }
    }

    fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(format!("{}:{}", self.server_address, self.port))
    }

    pub fn run(&self) -> io::Result<()> {
        let mut stream = self.connect()?;
        log::info!("Connected to server on {}", stream.peer_addr()?);
        if let Some(ref pwd) = self.password {
            stream.write_all(pwd.as_bytes())?;
        }

        let mut buffer = [0; 1024];
        let mut clipboard_listener =
            ClipboardListener::new().expect("Failed to create clipboard listener");

        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        log::info!("Server closed the connection. Exiting.");
                        break;
                    }
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    log::debug!("Received from server: {}", message);

                    let _ = clipboard_listener.set_clipboard(&message);

                    thread::sleep(Duration::from_secs(1));
                }
                Err(e) => {
                    log::error!("Failed to receive data: {}", e);
                    //TODO: handle the error, with a retray or something
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        Ok(())
    }
}
