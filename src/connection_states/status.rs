use super::ConnectionState;
use super::ConnectionStateTag;
use super::ConnectionStateTransition;

use crate::chat::Chat;
use crate::error_type::ErrorType;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::server::Server;

use std::convert::TryInto;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

pub struct StatusState {}

impl ConnectionState for StatusState {
    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: Arc<Mutex<TcpStream>>,
        server: Arc<Mutex<Server>>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        println!("S: {:#?}", packet);
        match packet {
            ServerboundPacket::StatusRequest(_packet) => {
                let server_lock = server
                    .lock()
                    .map_err(|e| ErrorType::Fatal(format!("Could not lock server: {:?}", e)))?;
                ClientboundPacket::StatusResponse(StatusResponsePacket {
                    version_name: server_lock.settings.version.to_string(),
                    version_protocol: server_lock.settings.protocol_version,
                    players_max: server_lock.settings.max_players.try_into().unwrap(),
                    players_curr: 0,
                    sample: vec![],
                    description: Chat::new(server_lock.settings.motd.to_string()),
                })
                .writer()
                .write(stream)?;
                Ok(ConnectionStateTransition::Remain)
            }
            ServerboundPacket::Ping(packet) => {
                ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload,
                })
                .writer()
                .write(stream)?;
                Ok(ConnectionStateTransition::TransitionTo(
                    ConnectionStateTag::Exit,
                ))
            }
            x => Err(ErrorType::Fatal(format!(
                "Unsupported packet in Status state: {:#?}",
                x
            ))),
        }
    }
}
