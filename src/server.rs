use std::io::{self, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::thread;
use std::time::Duration;

use crate::clipboard_listener::ClipboardListener;

use self::errors::ServerError;
pub mod errors;

#[derive(Debug, Clone)]
pub struct Server {
    address: String,
    port: u16,
}

impl Server {
    pub fn new(address: &str, port: u16) -> Result<Self, ServerError> {
        let full_address = format!("{}:{}", address, port);
        if full_address.to_socket_addrs().is_ok() {
            Ok(Server {
                address: address.to_string(),
                port,
            })
        } else {
            Err(ServerError::new("Invalid address"))
        }
    }

    fn listen(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.address, self.port))
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut rclip = ClipboardListener::new()
            .map_err(|e| println!("{}", e))
            .expect("Failed to create clipboard listener");
        loop {
            if rclip.have_new_clip() {
                let message = rclip.get_new_clip().unwrap_or(String::new());
                if let Err(e) = stream.write_all(message.as_bytes()) {
                    println!("Failed to send data: {}", e);
                    thread::sleep(Duration::from_secs(1));
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = self.listen()?;

        println!("Server listening on {}:{}", self.address, self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr()?);
                    let server_clone = self.clone();
                    thread::spawn(move || server_clone.handle_client(stream));
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}
