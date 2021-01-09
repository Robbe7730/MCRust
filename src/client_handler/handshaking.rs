use super::ConnectionState;
use super::ConnectionStateTrait;
use super::ConnectionStateTag;
use super::ConnectionStateTransition;

use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::Server;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct HandshakingState {}

impl ConnectionStateTrait for HandshakingState {
    fn from_state(
        _state: ConnectionState,
    ) -> Result<Self, ErrorType> {
        Err(ErrorType::Fatal(format!(
            "Cannot go back into Handshaking state"
        )))
    }

    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: TcpStream,
        server: Arc<Server>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("H: {:#?}", packet);
        match packet {
            ServerboundPacket::LegacyPing(_packet) => {
                let server_lock = server
                    .data
                    .lock()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;
                let packet = ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                    protocol_version: server_lock.settings.protocol_version,
                    minecraft_version: server_lock.settings.version.to_string(),
                    motd: server_lock.settings.motd.to_string(),
                    curr_player_count: 0, //TODO
                    max_player_count: server_lock.settings.max_players.try_into().unwrap(),
                });
                packet.writer().write(stream)?;
                Ok(ConnectionStateTransition::TransitionTo(
                    ConnectionStateTag::Exit,
                ))
            }
            ServerboundPacket::Handshaking(packet) => {
                Ok(ConnectionStateTransition::TransitionTo(packet.next_state))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Handshaking state: {:#?}",
                x
            ))),
        }
    }
}
