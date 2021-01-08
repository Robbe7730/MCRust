mod chat;
mod error_type;
mod nbt;
mod packets;
mod server;
mod util;
mod connection_states;

use connection_states::ClientHandler;
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
            ClientHandler::run(stream.expect("Invalid stream"));
        });
    }
}
