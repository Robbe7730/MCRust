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
