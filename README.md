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

## Setting Up Neovim and Vim for Remote Clipboard Sync

Neovim/Vim Configuration
To enable clipboard synchronization with Neovim or Vim, you need to configure an autocommand to write the yanked text to a file. This file will be monitored by the Remote Clipboard Sync tool on the server.

1. Configure Neovim/Vim:
   Open your init.vim/init.lua file for Neovim or .vimrc file for Vim and add the following lines:

```vim
" Autocommand that triggers when text is yanked
augroup YankToFile
  autocmd!
  autocmd TextYankPost * call writefile([@0], '/path/to/your/yankfile.txt')
augroup END
```

Replace /path/to/your/yankfile.txt with the actual path to the file that Remote Clipboard Sync will monitor.

- Example

```vim
" Autocommand that triggers when text is yanked
augroup YankToFile
  autocmd!
  autocmd TextYankPost * call writefile([@0], '/home/ubuntu/yankfile.txt')
augroup END
```

2. Ensure File Accessibility:
   Make sure that the file specified in clipboard_file is accessible and writable by both Neovim/Vim and the Remote Clipboard Sync tool.

Using with Remote Clipboard Sync
Once you have configured Neovim/Vim as described above, any text yanked (copied) in Neovim/Vim will be written to the specified file. The Remote Clipboard Sync tool running on your server will monitor this file and synchronize the clipboard contents accordingly.
