use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTrait;
use super::ConnectionStateTransition;

use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::Entity;
use crate::Server;

use std::convert::TryInto;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct HandshakingState {}

impl ConnectionStateTrait for HandshakingState {
    fn from_state(_state: ConnectionState) -> Result<Self, ErrorType> {
        Err(ErrorType::Fatal(format!(
            "Cannot go back into Handshaking state"
        )))
    }

    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        server: Arc<Server>,
    ) -> Result<(Vec<ClientboundPacket>, ConnectionStateTransition), ErrorType> {
        let mut queue = vec![];
        match packet {
            ServerboundPacket::LegacyPing(_packet) => {
                let server_lock = server
                    .data
                    .lock()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;

                let entities_lock = server_lock
                    .entities
                    .read()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock entities: {:?}", e)))?;

                let player_count = entities_lock
                    .values()
                    .filter_map(|e| {
                        let entity_lock = e.read();
                        if entity_lock.is_err() {
                            None
                        } else {
                            #[allow(irrefutable_let_patterns)]
                            if let Entity::PlayerEntity(_) = entity_lock.unwrap().clone() {
                                Some(1)
                            } else {
                                None
                            }
                        }
                    })
                    .count();

                queue.push(ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                    protocol_version: server_lock.settings.protocol_version,
                    minecraft_version: server_lock.settings.version.to_string(),
                    motd: server_lock.settings.motd.to_string(),
                    curr_player_count: player_count,
                    max_player_count: server_lock.settings.max_players.try_into().unwrap(),
                }));
                Ok((queue, ConnectionStateTransition::TransitionTo(
                    ConnectionStateTag::Exit,
                )))
            }
            ServerboundPacket::Handshaking(packet) => {
                Ok((queue, ConnectionStateTransition::TransitionTo(packet.next_state)))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Handshaking state: {:#?}",
                x
            ))),
        }
    }
}
