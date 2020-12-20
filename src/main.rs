use std::net::TcpListener;

mod client_handler;
mod error_type;
mod packet_reader;
mod packet_writer;
mod packets;
mod structs;

use client_handler::ClientHandler;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ServerStatus {
    version: String,
    protocol_version: usize,
    max_players: usize,
    motd: String,
}

pub struct Server {
    status: ServerStatus,
}

impl Server {
    pub fn new() -> Self {
        Server {
            status: ServerStatus {
                version: format!("MCRust 0.1.0"),
                protocol_version: 498,
                max_players: 20,
                motd: format!("Hello from Rust"),
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
