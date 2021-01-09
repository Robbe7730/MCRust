mod chat;
mod client_handler;
mod error_type;
mod nbt;
mod packets;
mod server;
mod util;

use client_handler::ClientHandler;
use server::ServerData;
use packets::clientbound::ClientboundPacket;


use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub type Eid = u32;

pub struct Server {
    data: Arc<Mutex<ServerData>>,
    connections: Arc<Mutex<HashMap<usize, Arc<ClientHandler>>>>,
}

impl Server {
    pub fn run(self) {
        let server_arc = Arc::new(self);
        let listener = TcpListener::bind("127.0.0.1:25565").expect("Could not start server");
        let mut curr_id = 0;

        for stream in listener.incoming() {
            let server_copy = server_arc.clone();
            let connections_copy = server_copy.connections.clone();
            let connection_id = curr_id;
            curr_id += 1;
            thread::spawn(move || {
                let client_handler = ClientHandler::new(stream.expect("Invalid stream"), server_copy);
                let ch_arc = Arc::new(client_handler);
                connections_copy
                    .lock()
                    .expect("Could not lock connection list")
                    .insert(connection_id, ch_arc.clone());
                ch_arc.run();
                connections_copy
                    .lock()
                    .expect("Could not lock connection list")
                    .remove(&connection_id);
            });
        }
    }
    
    pub fn send_to_all(&self, packet: ClientboundPacket) {
        self.connections
            .lock()
            .expect("Could not lock connection table")
            .values()
            .for_each(|x| x.send_packet(packet.clone()).unwrap());
    }
}

fn main() {
    let server = Server {
        data: Arc::new(Mutex::new(ServerData::new())),
        connections: Arc::new(Mutex::new(HashMap::new())),
    };
    server.run();
}
