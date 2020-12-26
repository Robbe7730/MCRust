use std::net::TcpListener;

mod client_handler;
mod error_type;
mod nbt;
mod packets;
mod structs;

use client_handler::ClientHandler;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ServerSettings {
    version: String,
    protocol_version: usize,
    max_players: usize,
    motd: String,
    online: bool,
}

pub struct Server {
    settings: ServerSettings,
}

impl Server {
    pub fn new() -> Self {
        Server {
            settings: ServerSettings {
                version: format!("MCRust 0.1.0"),
                protocol_version: 498,
                max_players: 20,
                motd: format!("Hello from Rust"),
                online: false,
            },
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Could not start server");

    let server = Arc::new(Mutex::new(Server::new()));

    for stream in listener.incoming() {
        let server_copy = server.clone();
        thread::spawn(|| {
            ClientHandler::new(stream.expect("Invalid stream"), server_copy).run();
        });
    }
}
