use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTrait;
use super::ConnectionStateTransition;

use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::Server;
use crate::server::World;
use crate::util::offline_player_uuid;
use crate::Eid;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, PartialEq)]
pub struct LoginState {
    pub player_eid: Eid,
}

impl ConnectionStateTrait for LoginState {
    fn from_state(prev_state: ConnectionState) -> Result<Self, ErrorType> {
        match prev_state {
            ConnectionState::Handshaking(_) => Ok(Self { player_eid: 0 }),
            x => Err(ErrorType::Fatal(format!(
                "Cannot go into Login state from {:#?}",
                x
            ))),
        }
    }

    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: Arc<Mutex<TcpStream>>,
        server: Arc<Mutex<Server>>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("L: {:#?}", packet);
        match packet {
            ServerboundPacket::LoginStart(packet) => {
                let server_lock = server
                    .lock()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;
                let uuid;

                // Create uuid based on online mode
                if server_lock.settings.online {
                    return Err(ErrorType::Fatal(format!("Online mode is not implemented")));
                } else {
                    uuid = offline_player_uuid(&packet.username);
                }

                // First reply
                let resp_packet = ClientboundPacket::LoginSuccess(LoginSuccessPacket {
                    username: packet.username.clone(),
                    uuid: uuid::Uuid::nil(),
                });
                println!("PING {:?}", resp_packet.writer());
                resp_packet.writer()
                .write(stream.clone())?;


                // Create and load a new player
                self.player_eid = server_lock.load_or_create_player(&packet.username, uuid)?;
                let entity_arc =
                    server_lock
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
                let world: &World = server_lock
                    .settings
                    .worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                // For borrowing reasons, these values need te be stored before calling
                // self.send_packet
                let hashed_seed = u64::from_be_bytes(world.seed[0..8].try_into().unwrap());
                let reduced_debug_info = world.reduced_debug_info;
                let enable_respawn_screen = world.enable_respawn_screen;
                let is_debug = world.is_debug;
                let is_flat = world.is_flat;

                ClientboundPacket::JoinGame(JoinGamePacket {
                    entity_id: self.player_eid,
                    is_hardcore: server_lock.settings.is_hardcore,
                    gamemode,
                    previous_gamemode,
                    world_names: server_lock
                        .settings
                        .worlds
                        .keys()
                        .map(|x| x.to_string())
                        .collect(),
                    dimension_codec: server_lock.dimension_codec.clone(),
                    dimension,
                    world_name: server_lock.settings.selected_world.clone(),
                    hashed_seed,
                    max_players: server_lock.settings.max_players,
                    view_distance: server_lock.settings.view_distance,
                    reduced_debug_info,
                    enable_respawn_screen,
                    is_debug,
                    is_flat,
                })
                .writer()
                .write(stream)?;
                Ok(ConnectionStateTransition::TransitionTo(
                    ConnectionStateTag::Play,
                ))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Login state: {:#?}",
                x
            ))),
        }
    }
}
