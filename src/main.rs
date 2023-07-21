mod chat;
mod client_handler;
mod error_type;
mod nbt;
mod packets;
mod server;
mod world;
mod player;

use client_handler::ClientHandler;
use client_handler::ConnectionState;
use packets::clientbound::ClientboundPacket;
use packets::clientbound::KeepAlivePacket;
use server::ServerData;

use std::collections::HashMap;
use std::net::TcpListener;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

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
            let twenty_seconds = Duration::new(20, 0);
            loop {
                thread::sleep(twenty_seconds);
                server_arc_copy.send_keepalive();
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

    pub fn send_keepalive(&self) {
        self.connections
            .lock()
            .expect("Could not lock connection table")
            .values()
            .for_each(|x| {
                // This whole function is to get the player object for the ClientHandler
                // This is a mess...
                let player_eid;
                {
                    let state_lock = x.state.lock().expect("Could not lock state");
                    if let ConnectionState::Handshaking(_) = *state_lock {
                        return;
                    }
                    if let ConnectionState::Status(_) = *state_lock {
                        return;
                    }
                    player_eid = match state_lock.deref() {
                        ConnectionState::Play(playstate) => playstate.player_eid,
                        ConnectionState::Login(loginstate) => loginstate.player_eid,
                        _ => unreachable!()
                    };
                }
                let maybe_server_data_lock = self.data.lock();

                if maybe_server_data_lock.is_err() {
                    eprintln!("Could not lock server data");
                    return;
                }

                let server_data_lock = maybe_server_data_lock.unwrap();
                let world = server_data_lock.settings.worlds
                    .get(&server_data_lock.settings.selected_world)
                    .expect("Invalid world selected");

                let maybe_entity_arc = world.get_entity(player_eid);
                if maybe_entity_arc.is_err() {
                    eprintln!("Player could not be found");
                    return;
                }
                let maybe_entity_res = maybe_entity_arc.unwrap();
                if maybe_entity_res.is_none() {
                    eprintln!("Player does not exist");
                    return;
                }
                let maybe_entity = maybe_entity_res.unwrap();
                let maybe_entity_lock = maybe_entity.write();
                if maybe_entity_lock.is_err() {
                    eprintln!("Could not lock player for writing");
                    return
                }
                let mut maybe_player_entity = maybe_entity_lock.unwrap();
                let maybe_player = maybe_player_entity.as_player_mut();
                if maybe_player.is_err() {
                    eprintln!("Could not load player");
                    return
                }
                let player = maybe_player.unwrap();

                let packet = ClientboundPacket::KeepAlive(KeepAlivePacket::for_player(player));
                x.send_packet(packet.clone()).unwrap()
            });
    }
}

fn main() {
    let server = Server {
        data: Arc::new(Mutex::new(ServerData::new())),
        connections: Arc::new(Mutex::new(HashMap::new())),
    };
    server.run();
}
