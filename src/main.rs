#![allow(dead_code)]

mod client;
mod clipboard_listener;
mod server;

use std::process;

use crate::client::Client;
use crate::server::Server;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Remote Clipboard Sync")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Synchronizes clipboard contents between a remote server and a local machine.")
        .arg(
            Arg::with_name("role")
                .help("The role to run ('server' or 'client')")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("address")
                .help("The address to bind/connect to")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("port")
                .help("The port to bind/connect to")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::with_name("clipboard_file")
                .help("The path to the clipboard file (only required for server)")
                .required(false)
                .index(4),
        )
        .get_matches();

    let role = matches.value_of("role").unwrap();
    let address = matches.value_of("address").unwrap();
    let port = matches
        .value_of("port")
        .unwrap()
        .parse::<u16>()
        .expect("Invalid port number");

    match role {
        "server" => {
            let clipboard_file = matches
                .value_of("clipboard_file")
                .expect("Clipboard file is required for server");
            let server = Server::new(address, port, clipboard_file)
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
