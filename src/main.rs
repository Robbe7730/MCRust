use std::net::TcpListener;

mod client_handler;
mod packet_reader;
mod packet_writer;
mod packets;
mod structs;

use client_handler::ClientHandler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Could not start server");

    for stream in listener.incoming() {
        if let Err(x) = ClientHandler::new(stream.expect("Invalid stream")).run() {
            println!("Stream ended: {}", x);
        }
    }
}
