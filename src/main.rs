#![allow(dead_code)]

mod client;
mod clipboard_listener;
mod server;

use std::env;
use std::process;

use crate::client::Client;
use crate::server::Server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: program [server|client] <address> <port>");
        process::exit(1);
    }

    let role = &args[1];
    let address = &args[2];
    let port = args[3].parse::<u16>().expect("Invalid port number");

    match role.as_str() {
        "server" => {
            let server = Server::new(address, port)
                .expect("Failed to create server with given address and port");
            server.run()?;
        }
        "client" => {
            let client = Client::new(address, port)
                .expect("Failed to create client with given server address and port");
            client.run()?;
        }
        _ => {
            eprintln!("Invalid role. Use 'server' or 'client'.");
            process::exit(1);
        }
    }

    Ok(())
}
