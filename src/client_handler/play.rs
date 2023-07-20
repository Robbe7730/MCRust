use super::ConnectionState;
use super::ConnectionStateTrait;
use super::ConnectionStateTransition;

use crate::chat::Chat;
use crate::chat::ChatPosition;
use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::nbt::NBTReader;
use crate::packets::serverbound::ServerboundPacket;
use crate::Eid;
use crate::Server;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;

use rand::random;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct PlayState {
    player_eid: Eid,
}

impl ConnectionStateTrait for PlayState {
    fn from_state(prev_state: ConnectionState) -> Result<Self, ErrorType> {
        match prev_state {
            ConnectionState::Login(login_state) => Ok(Self {
                player_eid: login_state.player_eid,
            }),
            x => Err(ErrorType::Fatal(format!(
                "Cannot go into Play state from {:#?}",
                x
            ))),
        }
    }

    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: TcpStream,
        server: Arc<Server>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("P: {:#?}", packet);
        let server_lock = server
            .data
            .lock()
            .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;
        match packet {
            ServerboundPacket::ClientSettings(_packet) => {
                // Get the player
                let entity_arc = server_lock
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
                ClientboundPacket::HeldItemChange(HeldItemChangePacket { slot })
                    .writer()
                    .write(stream.try_clone().map_err(|e| {
                        ErrorType::Fatal(format!("Could not clone TCP stream: {}", e))
                    })?)?;

                // Send the player position and look packet
                let x = player.position.x.round() as i64;
                let y = player.position.y.round() as i64;
                let z = player.position.z.round() as i64;
                let pitch = player.look.pitch;
                let yaw = player.look.yaw;
                ClientboundPacket::PlayerPositionAndLook(PlayerPositionAndLookPacket {
                    x: ValueType::Absolute(x),
                    y: ValueType::Absolute(y),
                    z: ValueType::Absolute(z),
                    yaw: ValueType::Absolute(yaw),
                    pitch: ValueType::Absolute(pitch),
                    teleport_id: random(),
                })
                .writer()
                .write(stream.try_clone().map_err(|e| {
                    ErrorType::Fatal(format!("Could not clone TCP stream: {}", e))
                })?)?;

                // Send a test message
                ClientboundPacket::ChatMessage(ChatMessagePacket {
                    message: Chat::new("Welcome to the server".to_string()),
                    sender: Uuid::nil(),
                    position: ChatPosition::SystemMessage,
                })
                .writer()
                .write(stream.try_clone().map_err(|e| {
                    ErrorType::Fatal(format!("Could not clone TCP stream: {}", e))
                })?)?;
                Ok(ConnectionStateTransition::Remain)
            }
            ServerboundPacket::ChatMessage(packet) => {
                // Get the player
                let entity_arc = server_lock
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let entity = entity_arc.read().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for reading: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player()?;

                // Send the message to all players
                let chat_packet = ClientboundPacket::ChatMessage(ChatMessagePacket {
                    message: Chat::new(format!("<{}> {}", player.username, packet.message)),
                    sender: player.uuid,
                    position: ChatPosition::SystemMessage,
                });
                server.send_to_all(chat_packet);
                Ok(ConnectionStateTransition::Remain)
            }
            ServerboundPacket::KeepAlive(_packet) => {
                // TODO: validate response id == sent response id
                Ok(ConnectionStateTransition::Remain)
            }
            ServerboundPacket::PluginMessage(packet) => {
                match packet.channel.as_str() {
                    "minecraft:brand" => {
                        println!(
                            "Player {} is using brand {}",
                            self.player_eid,
                            NBTReader::new(
                                packet.data.as_slice(),
                                packet.data.len().try_into().map_err(|_|
                                    ErrorType::Recoverable(format!("Packet data too big"))
                                )?
                            ).read_string()?
                        );
                        Ok(ConnectionStateTransition::Remain)
                    }
                    c => {
                        Err(ErrorType::Recoverable(format!("Unimplemented channel {}" , c)))
                    }
                }
            }
            ServerboundPacket::TeleportConfirm(_packet) => {
                // TODO: validate teleport id == sent teleport id
                Ok(ConnectionStateTransition::Remain)
            }
            ServerboundPacket::PlayerPositionAndRotation(packet) => {
                // Get the player
                let entity_arc = server_lock
                    .get_entity(self.player_eid)?
                    .ok_or(ErrorType::Fatal("Player does not exist".to_string()))?;
                let mut entity = entity_arc.write().map_err(|e| {
                    ErrorType::Fatal(format!(
                        "Could not lock player for reading: {}",
                        e.to_string()
                    ))
                })?;
                let player = entity.as_player_mut()?;

                player.position.x = packet.x;
                player.position.y = packet.feet_y;
                player.position.z = packet.z;
                player.look.yaw = packet.yaw;
                player.look.pitch = packet.pitch;
                player.position.on_ground = packet.on_ground;

                Ok(ConnectionStateTransition::Remain)
            }
            x => Err(ErrorType::Recoverable(format!(
                "Unimplemented packet in Play state: {:#?}",
                x
            ))),
        }
    }
}
