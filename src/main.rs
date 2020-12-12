use std::net::TcpListener;

mod packet_reader;
mod client_handler;
mod packets;

use client_handler::ClientHandler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565")
                            .expect("Could not start server");

    for stream in listener.incoming() {
        ClientHandler::new(stream.expect("Invalid stream")).run();
    }
}
