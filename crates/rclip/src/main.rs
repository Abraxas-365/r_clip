use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;

mod client;
mod clipboard_listener;
mod server;

#[derive(ValueEnum, Default, Debug, Clone)]
enum Role {
    #[default]
    Client,
    Server,
}

#[derive(Debug, Parser)]
#[command(name = "v0.1.0")]
#[command(about = "Synchronizes clipboard contents between a remote server and a local machine.")]
#[command(
    author = "Luis Fernando Miranda <luisfmiranda8@gmail.com>, Guzmán Monné <guzman.monne@cloudbridge.com.uy>"
)]
pub struct Cli {
    /// The role to run.
    #[arg(short, long, value_enum)]
    role: Role,
    /// The address to bind/connect
    #[arg(short, long)]
    address: String,
    /// The port to bind/connect
    #[arg(short, long)]
    port: u16,
    /// The path to the clipboard file (only required for server)
    #[arg(short, long)]
    clipboard_file: Option<String>,

    /// The password to use for the server
    #[arg(short, long)]
    password: Option<String>,
}

use crate::client::Client;
use crate::server::Server;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.role {
        Role::Server => {
            let server = Server::new(
                &cli.address,
                cli.port,
                &cli.clipboard_file
                    .expect("A clipboard file should be provided in server mode"),
                cli.password,
            )
            .expect("Failed to create server with given address and port");
            server.run()?;
        }
        Role::Client => {
            let client = Client::new(&cli.address, cli.port, cli.password)
                .expect("Failed to create client with given server address and port");
            client.run()?;
        }
    }

    Ok(())
}
