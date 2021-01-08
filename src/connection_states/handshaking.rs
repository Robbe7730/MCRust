use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTransition;

use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

pub struct HandshakingState {}

impl ConnectionState for HandshakingState {
    fn handle_packet(
        &self,
        packet: ServerboundPacket,
        stream: Arc<Mutex<TcpStream>>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("H: {:#?}", packet);
        match packet {
            ServerboundPacket::LegacyPing(packet) => {
                //let packet = ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                //protocol_version: self.server.settings.protocol_version,
                //minecraft_version: self.server.settings.version.to_string(),
                //motd: self.server.settings.motd.to_string(),
                //curr_player_count: 0,
                //max_player_count: self.server.settings.max_players.try_into().unwrap(),
                //});
                let packet = ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                    protocol_version: 479,
                    minecraft_version: format!("MCRust 0.1.0"),
                    motd: format!("Hello from Rust"),
                    curr_player_count: 0,
                    max_player_count: 5,
                });
                packet.writer().write(stream);
                Ok(ConnectionStateTransition::TransitionTo(ConnectionStateTag::Exit))
            }
            ServerboundPacket::Handshaking(packet) => {
                Ok(ConnectionStateTransition::TransitionTo(packet.next_state))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Handshaking mode: {:#?}",
                x
            ))),
        }
    }
}
