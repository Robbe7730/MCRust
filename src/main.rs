mod chat;
mod client_handler;
mod error_type;
mod nbt;
mod packets;
mod server;
mod util;

use client_handler::ClientHandler;
use packets::clientbound::ClientboundPacket;
use server::ServerData;

use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub type Eid = u32;

pub struct Server {
    data: Arc<Mutex<ServerData>>,
    connections: Arc<Mutex<HashMap<usize, Arc<ClientHandler>>>>,
}

impl Server {
    pub fn run(self) {
        let server_arc = Arc::new(self);

        // Set up keepalive ticks (in the future this could be game ticks)
        let server_arc_copy = server_arc.clone();
        thread::spawn(move || {
            use crate::packets::clientbound::KeepAlivePacket;
            let twenty_seconds = Duration::new(20, 0);
            loop {
                thread::sleep(twenty_seconds);
                server_arc_copy.send_to_all(ClientboundPacket::KeepAlive(KeepAlivePacket {
                    id: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Invalid system time").as_secs() as i64,
                }));
            }
        });

        // Set up client listener
        let listener = TcpListener::bind("0.0.0.0:25565").expect("Could not start server");
        let mut curr_id = 0;

        for stream in listener.incoming() {
            let server_copy = server_arc.clone();
            let connections_copy = server_copy.connections.clone();
            let connection_id = curr_id;
            curr_id += 1;
            thread::spawn(move || {
                let client_handler =
                    ClientHandler::new(stream.expect("Invalid stream"), server_copy);
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
