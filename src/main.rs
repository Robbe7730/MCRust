mod chat;
mod client_handler;
mod error_type;
mod nbt;
mod packets;
mod server;
mod util;

use client_handler::ClientHandler;
use server::Server;

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

pub type Eid = u32;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Could not start server");

    let server = Arc::new(Server::new());

    for stream in listener.incoming() {
        let server_copy = server.clone();
        thread::spawn(|| {
            ClientHandler::new(stream.expect("Invalid stream"), server_copy).run();
        });
    }
}
