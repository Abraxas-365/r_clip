use std::io::{self, Write};
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
}

impl Server {
    pub fn new(address: &str, port: u16, clipboard_file: &str) -> Result<Self, ServerError> {
        let full_address = format!("{}:{}", address, port);
        if full_address.to_socket_addrs().is_ok() {
            Ok(Server {
                address: address.to_string(),
                port,
                clipboard_file: clipboard_file.to_string(),
            })
        } else {
            Err(ServerError::new("Invalid address"))
        }
    }

    fn listen(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(format!("{}:{}", self.address, self.port))
    }

    fn handle_client(&self, mut stream: TcpStream) {
        fs::write(&self.clipboard_file, "").expect("Failed to clear clipboard file");
        loop {
            let current_clip =
                fs::read_to_string(&self.clipboard_file).unwrap_or_else(|_| String::new());

            if !current_clip.is_empty() {
                if let Err(e) = stream.write_all(current_clip.as_bytes()) {
                    println!("Failed to send data: {}", e);
                    thread::sleep(Duration::from_secs(1));
                } else {
                    fs::write(&self.clipboard_file, "").expect("Failed to clear clipboard file");
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
