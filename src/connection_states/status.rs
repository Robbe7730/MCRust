use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTransition;

use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::chat::Chat;

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

pub struct StatusState {}

impl ConnectionState for StatusState {
    fn handle_packet(
        &self,
        packet: ServerboundPacket,
        stream: Arc<Mutex<TcpStream>>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("S: {:#?}", packet);
        match packet {
            ServerboundPacket::StatusRequest(_packet) => {
                ClientboundPacket::StatusResponse(StatusResponsePacket {
                    //TODO
                    //version_name: self.server.settings.version.to_string(),
                    //version_protocol: self.server.settings.protocol_version,
                    //players_max: self.server.settings.max_players.try_into().unwrap(),
                    //players_curr: 0,
                    //sample: vec![],
                    //description: Chat::new(self.server.settings.motd.to_string()),
                    version_name: "MCRust 0.1.0".to_string(),
                    version_protocol: 497,
                    players_max: 5,
                    players_curr: 0,
                    sample: vec![],
                    description: Chat::new("Hello from Rust".to_string()),
                }).writer().write(stream)?;
                Ok(ConnectionStateTransition::Remain)
            }
            ServerboundPacket::Ping(packet) => {
                ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload,
                }).writer().write(stream)?;
                Ok(ConnectionStateTransition::TransitionTo(ConnectionStateTag::Exit))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Status mode: {:#?}",
                x
            ))),
        }
    }
}
