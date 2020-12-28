use crate::chat::Chat;
use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::packet_reader::PacketReader;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::World;
use crate::util::offline_player_uuid;
use crate::Server;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

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
    server: Arc<Server>,
}

impl ClientHandler {
    pub fn new(stream: TcpStream, server: Arc<Server>) -> Self {
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
                println!("{:#?}", packet);
                let packet = ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                    protocol_version: self.server.settings.protocol_version,
                    minecraft_version: self.server.settings.version.to_string(),
                    motd: self.server.settings.motd.to_string(),
                    curr_player_count: 0,
                    max_player_count: self.server.settings.max_players,
                });
                self.send_packet(packet)?;
                Err(ErrorType::GracefulExit)?
            }
            ServerboundPacket::Handshaking(packet) => {
                println!("{:#?}", packet);
                self.state = packet.next_state;
            }
            ServerboundPacket::StatusRequest(packet) => {
                println!("{:#?}", packet);
                let packet = ClientboundPacket::StatusResponse(StatusResponsePacket {
                    version_name: self.server.settings.version.to_string(),
                    version_protocol: self.server.settings.protocol_version,
                    players_max: self.server.settings.max_players,
                    players_curr: 0,
                    sample: vec![],
                    description: Chat::new(self.server.settings.motd.to_string()),
                });
                self.send_packet(packet)?;
            }
            ServerboundPacket::Ping(packet) => {
                println!("{:#?}", packet);
                self.send_packet(ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload,
                }))?;
                Err(ErrorType::GracefulExit)?
            }
            ServerboundPacket::LoginStart(packet) => {
                println!("{:#?}", packet);
                let uuid;

                // Create uuid based on online mode
                if self.server.settings.online {
                    return Err(ErrorType::Fatal(format!("Online mode is not implemented")));
                } else {
                    uuid = offline_player_uuid(&packet.username);
                }

                // First reply
                self.send_packet(ClientboundPacket::LoginSuccess(LoginSuccessPacket {
                    username: packet.username.clone(),
                    uuid: uuid,
                }))?;

                // Switch to play state
                self.state = ConnectionState::Play;

                // Create and load a new player
                let eid = self.server.load_or_create_player(&packet.username, uuid)?;
                let player;
                let entity_arc = self.server.get_entity(eid)?.ok_or(ErrorType::Fatal(
                    "Newly created player entity does not exist".to_string(),
                ))?;
                let entity = entity_arc.read().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for reading: {}",
                        e.to_string()
                    ))
                })?;
                player = entity
                    .as_player()
                    .ok_or(ErrorType::Fatal("Player is not a player.".to_string()))?;

                // Load the world and some its values
                let world: &World = self
                    .server
                    .settings
                    .worlds
                    .get(&self.server.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                // For borrowing reasons, these values need te be stored before calling
                // self.send_packet
                let hashed_seed = u64::from_be_bytes(world.seed[0..8].try_into().unwrap());
                let reduced_debug_info = world.reduced_debug_info;
                let enable_respawn_screen = world.enable_respawn_screen;
                let is_debug = world.is_debug;
                let is_flat = world.is_flat;

                self.send_packet(ClientboundPacket::JoinGame(JoinGamePacket {
                    entity_id: eid,
                    is_hardcore: self.server.settings.is_hardcore,
                    gamemode: player.gamemode.clone(),
                    previous_gamemode: player.previous_gamemode.clone(),
                    world_names: self
                        .server
                        .settings
                        .worlds
                        .keys()
                        .map(|x| x.to_string())
                        .collect(),
                    dimension_codec: self.server.dimension_codec.clone(),
                    dimension: player.dimension.clone(),
                    world_name: self.server.settings.selected_world.clone(),
                    hashed_seed,
                    max_players: self.server.settings.max_players,
                    view_distance: self.server.settings.view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                }))?;
            }
            ServerboundPacket::ClientSettings(packet) => {
                println!("{:#?}", packet);
            }
        })
    }
}
