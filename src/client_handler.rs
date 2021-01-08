use crate::chat::Chat;
use crate::chat::ChatPosition;
use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::packet_reader::PacketReader;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::Server;
use crate::server::World;
use crate::util::offline_player_uuid;
use crate::Eid;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use rand::random;
use uuid::Uuid;

pub struct ClientHandler {
    stream: Arc<Mutex<TcpStream>>,
    state: ConnectionState,
    reader: PacketReader,
    server: Arc<Server>,
    player_eid: Eid,
}

impl ClientHandler {
    pub fn send_packet(&mut self, packet: ClientboundPacket) -> Result<(), ErrorType> {
        packet.writer().write(self.stream.clone())
    }

    fn handle_packet(&mut self, packet: ServerboundPacket) -> Result<(), ErrorType> {
        // read_packet has already filtered out incorrect states, so it is not neccesary here
        Ok(match packet {
            ServerboundPacket::StatusRequest(packet) => {
                println!("{:#?}", packet);
                let packet = ClientboundPacket::StatusResponse(StatusResponsePacket {
                    version_name: self.server.settings.version.to_string(),
                    version_protocol: self.server.settings.protocol_version,
                    players_max: self.server.settings.max_players.try_into().unwrap(),
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
                self.player_eid = self.server.load_or_create_player(&packet.username, uuid)?;
                let entity_arc =
                    self.server
                        .get_entity(self.player_eid)?
                        .ok_or(ErrorType::Fatal(
                            "Newly created player does not exist".to_string(),
                        ))?;
                let entity = entity_arc.read().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player entity for reading: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player()?;
                let gamemode = player.gamemode.clone();
                let previous_gamemode = player.previous_gamemode.clone();
                let dimension = player.dimension.clone();

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
                    entity_id: self.player_eid,
                    is_hardcore: self.server.settings.is_hardcore,
                    gamemode,
                    previous_gamemode,
                    world_names: self
                        .server
                        .settings
                        .worlds
                        .keys()
                        .map(|x| x.to_string())
                        .collect(),
                    dimension_codec: self.server.dimension_codec.clone(),
                    dimension,
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
                // Get the player
                let entity_arc = self
                    .server
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let entity = entity_arc.read().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for reading: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player()?;

                // Send the held item change packet
                let slot = player.selected_slot;
                self.send_packet(ClientboundPacket::HeldItemChange(HeldItemChangePacket {
                    slot,
                }))?;

                // Send the player position and look packet
                let x = player.position.x;
                let y = player.position.y;
                let z = player.position.z;
                let pitch = player.look.pitch;
                let yaw = player.look.yaw;
                self.send_packet(ClientboundPacket::PlayerPositionAndLook(
                    PlayerPositionAndLookPacket {
                        x: ValueType::Absolute(x),
                        y: ValueType::Absolute(y),
                        z: ValueType::Absolute(z),
                        yaw: ValueType::Absolute(yaw),
                        pitch: ValueType::Absolute(pitch),
                        teleport_id: random(),
                    },
                ))?;

                // Send a test message
                self.send_packet(ClientboundPacket::ChatMessage(ChatMessagePacket {
                    message: Chat::new("Welcome to the server".to_string()),
                    sender: Uuid::nil(),
                    position: ChatPosition::SystemMessage,
                }))?;
            }
        })
    }
}
