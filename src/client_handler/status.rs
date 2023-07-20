use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTrait;
use super::ConnectionStateTransition;

use crate::chat::Chat;
use crate::error_type::ErrorType;
use crate::packets::clientbound::status_response::StatusResponsePlayer;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::Entity;
use crate::Server;

use std::convert::TryInto;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct StatusState {}

impl ConnectionStateTrait for StatusState {
    fn from_state(prev_state: ConnectionState) -> Result<Self, ErrorType> {
        match prev_state {
            ConnectionState::Handshaking(_) => Ok(Self {}),
            x => Err(ErrorType::Fatal(format!(
                "Cannot go into Status state from {:#?}",
                x
            ))),
        }
    }

    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        server: Arc<Server>,
    ) -> Result<(Vec<ClientboundPacket>, ConnectionStateTransition), ErrorType> {
        let mut queue = vec![];
        match packet {
            ServerboundPacket::StatusRequest(_packet) => {
                let server_lock = server
                    .data
                    .lock()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;

                let world = server_lock
                    .settings
                    .worlds
                    .get(&server_lock.settings.selected_world)
                    .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;

                let entities_lock = world
                    .entities
                    .read()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock entities: {:?}", e)))?;

                let player_names = entities_lock
                    .values()
                    .filter_map(|e| {
                        let entity_lock = e.read();
                        if entity_lock.is_err() {
                            None
                        } else {
                            #[allow(irrefutable_let_patterns)]
                            if let Entity::PlayerEntity(p) = entity_lock.unwrap().clone() {
                                Some(p.clone())
                            } else {
                                None
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                queue.push(ClientboundPacket::StatusResponse(StatusResponsePacket {
                    version_name: server_lock.settings.version.to_string(),
                    version_protocol: server_lock.settings.protocol_version,
                    players_max: server_lock.settings.max_players.try_into().unwrap(),
                    players_curr: player_names.len(),
                    sample: player_names
                        .iter()
                        .map(|player| StatusResponsePlayer::from(player))
                        .collect(),
                    description: Chat::new(server_lock.settings.motd.to_string()),
                }));
                Ok((queue, ConnectionStateTransition::Remain))
            }
            ServerboundPacket::Ping(packet) => {
                queue.push(ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload,
                }));
                Ok((queue, ConnectionStateTransition::TransitionTo(
                    ConnectionStateTag::Exit,
                )))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Status state: {:#?}",
                x
            ))),
        }
    }
}
