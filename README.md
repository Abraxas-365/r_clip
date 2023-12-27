# RClip

## Overview

Remote Clipboard Sync is a tool designed to synchronize clipboard contents between a remote server and a local machine. This project is particularly useful for developers and system administrators who frequently work with remote environments using tools like nvim and need to copy-paste text between the remote server and their local machine. Whether it's copying error logs, code snippets, or configuration settings, Remote Clipboard Sync makes the process seamless and efficient.

## Features

- Cross-platform support for macOS and Linux.
- Real-time synchronization of clipboard contents between a remote server and a local machine.
- Easy to set up and use with minimal configuration.

## Installation

### Prerequisites

Rust and Cargo (Rust's package manager) must be installed on both the server and the local machine.
Network connectivity between the local machine and the remote server.

1. Clone the Repository:

```sh
git clone https://github.com/your-username/remote-clipboard-sync.git
cd remote-clipboard-sync
```

2. Build the Project:
   On both the server and the local machine, run:

```sh
cargo build --release
```

Executable:
The executable will be available under target/release/.

## Usage

### On the Server

Start the server component on your remote machine:

```sh
./r_clip server <server-address> <port>
```

Replace <server-address> with the server's IP address or hostname and <port> with the port number you want to use.

### On the Local Machine

Connect the client component to your server:

```sh
./r_clip client <server-address> <port>
```
