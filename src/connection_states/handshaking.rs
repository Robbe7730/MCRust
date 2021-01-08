use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTransition;

use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::Server;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

pub struct HandshakingState {}

impl ConnectionState for HandshakingState {
    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: Arc<Mutex<TcpStream>>,
        server: Arc<Mutex<Server>>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("H: {:#?}", packet);
        match packet {
            ServerboundPacket::LegacyPing(_packet) => {
                let server_lock = server
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
