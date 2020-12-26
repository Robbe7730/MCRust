use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use std::convert::TryInto;

use crate::chat::Chat;
use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::packet_reader::PacketReader;
use crate::packets::serverbound::ServerboundPacket;
use crate::util::offline_player_uuid;
use crate::Server;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl ConnectionState {
    pub fn from(i: isize) -> Result<Self, ErrorType> {
        match i {
            0 => Ok(ConnectionState::Handshaking),
            1 => Ok(ConnectionState::Status),
            2 => Ok(ConnectionState::Login),
            3 => Ok(ConnectionState::Play),
            x => Err(ErrorType::Fatal(format!("Invalid state {}", x))),
        }
    }
}

pub struct ClientHandler {
    stream: Arc<Mutex<TcpStream>>,
    state: ConnectionState,
    reader: PacketReader,
    server: Arc<Mutex<Server>>,
}

impl ClientHandler {
    pub fn new(stream: TcpStream, server: Arc<Mutex<Server>>) -> Self {
        let stream = Arc::new(Mutex::new(stream));
        Self {
            stream: stream.clone(),
            state: ConnectionState::Handshaking,
            reader: PacketReader::new(stream.clone()),
            server: server,
        }
    }

    pub fn run(&mut self) {
        println!("RUN!");
        loop {
            let packet_result = self
                .reader
                .read_packet(self.state)
                .and_then(|packet| self.handle_packet(packet));
            match packet_result {
                Ok(_) => println!("Another succesful packet handled"),
                Err(ErrorType::Fatal(msg)) => {
                    println!("FATAL: {}", msg);
                    self.graceful_exit();
                    break;
                }
                Err(ErrorType::Recoverable(msg)) => {
                    println!("Whoops: {}", msg);
                }
                Err(ErrorType::GracefulExit) => {
                    self.graceful_exit();
                    break;
                }
            }
        }
    }

    fn graceful_exit(&mut self) {
        // The connection is closed when it goes out of scope, so for now nothing needs to be done
        // here
        println!("Going down");
    }

    pub fn send_packet(&mut self, packet: ClientboundPacket) -> Result<(), ErrorType> {
        packet.writer().write(self.stream.clone())
    }

    fn handle_packet(&mut self, packet: ServerboundPacket) -> Result<(), ErrorType> {
        // read_packet has already filtered out incorrect states, so it is not neccesary here
        Ok(match packet {
            ServerboundPacket::LegacyPing(packet) => {
                println!("Legacy Ping @ {}:{}", packet.hostname, packet.port);
                let packet;
                {
                    let server_lock = self.server.lock().map_err(|e| {
                        ErrorType::Fatal(format!("Could not lock server {}", e.to_string()))
                    })?;
                    packet = ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                        protocol_version: server_lock.settings.protocol_version,
                        minecraft_version: server_lock.settings.version.to_string(),
                        motd: server_lock.settings.motd.to_string(),
                        curr_player_count: 0,
                        max_player_count: server_lock.settings.max_players,
                    })
                }
                self.send_packet(packet)?;
                Err(ErrorType::GracefulExit)?
            }
            ServerboundPacket::Handshaking(packet) => {
                println!(
                    "Handshake @ {}:{} version {} to state {:?}",
                    packet.server_address,
                    packet.server_port,
                    packet.protocol_version,
                    packet.next_state
                );
                self.state = packet.next_state;
            }
            ServerboundPacket::StatusRequest(_) => {
                println!("Status Request");
                let packet;
                {
                    let server_lock = self.server.lock().map_err(|e| {
                        ErrorType::Fatal(format!("Could not lock server {}", e.to_string()))
                    })?;
                    packet = ClientboundPacket::StatusResponse(StatusResponsePacket {
                        version_name: server_lock.settings.version.to_string(),
                        version_protocol: server_lock.settings.protocol_version,
                        players_max: server_lock.settings.max_players,
                        players_curr: 0,
                        sample: vec![],
                        description: Chat::new(server_lock.settings.motd.to_string()),
                    })
                }
                self.send_packet(packet)?;
            }
            ServerboundPacket::Ping(packet) => {
                println!("Ping with payload {}", packet.payload);
                self.send_packet(ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload,
                }))?;
                Err(ErrorType::GracefulExit)?
            }
            ServerboundPacket::LoginStart(packet) => {
                println!("Login start from {}", packet.username);
                let player;
                let uuid;
                let eid;
                let cloned_settings;
                let cloned_world;
                let dimension_codec;
                {
                    let mut server_lock = self.server.lock().map_err(|e| {
                        ErrorType::Fatal(format!("Could not lock server {}", e.to_string()))
                    })?;
                    uuid = offline_player_uuid(&packet.username);
                    let player_eid = server_lock.load_or_create_player(&packet.username, uuid)?;
                    player = player_eid.0;
                    eid = player_eid.1;
                    cloned_settings = server_lock.settings.clone();
                    cloned_world = server_lock.settings.worlds[&cloned_settings.selected_world].clone();
                    dimension_codec = server_lock.dimension_codec.clone();
                }
                if cloned_settings.online {
                    Err(ErrorType::Fatal(format!("Online mode is not implemented")))?
                } else {
                    self.send_packet(ClientboundPacket::LoginSuccess(LoginSuccessPacket {
                        username: packet.username,
                        uuid: uuid,
                    }))?;
                }
                self.state = ConnectionState::Play;
                self.send_packet(ClientboundPacket::JoinGame(JoinGamePacket {
                    entity_id: eid,
                    is_hardcore: cloned_settings.is_hardcore,
                    gamemode: player.gamemode.clone(),
                    previous_gamemode: player.previous_gamemode.clone(),
                    world_names: cloned_settings.worlds.keys().map(|x| x.to_string()).collect(),
                    dimension_codec: dimension_codec,
                    dimension: player.dimension.clone(),
                    world_name: cloned_settings.selected_world,
                    hashed_seed: u64::from_be_bytes(cloned_world.seed[0..8].try_into().unwrap()),
                    max_players: cloned_settings.max_players,
                    view_distance: cloned_settings.view_distance,
                    reduced_debug_info: cloned_world.reduced_debug_info,
                    enable_respawn_screen: cloned_world.enable_respawn_screen,
                    is_debug: cloned_world.is_debug,
                    is_flat: cloned_world.is_flat,
                }))?;
            }
        })
    }
}
