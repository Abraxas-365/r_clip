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
}

impl Client {
    pub fn new(server_address: &str, port: u16) -> Result<Self, ClientError> {
        let full_address = format!("{}:{}", server_address, port);
        if full_address.to_socket_addrs().is_ok() {
            Ok(Client {
                server_address: server_address.to_string(),
                port,
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
        println!("Successfully connected to the server");
        stream.write_all(b"Hello, server!")?;

        let mut buffer = [0; 1024];
        let mut clipboard_listener =
            ClipboardListener::new().expect("Failed to create clipboard listener");

        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        println!("Server closed the connection. Exiting.");
                        break;
                    }
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    println!("Received from server: {}", message);

                    let _ = clipboard_listener.set_clipboard(&message);

                    thread::sleep(Duration::from_secs(1));
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    //TODO: handle the error, with a retray or something
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }

        Ok(())
    }
}
