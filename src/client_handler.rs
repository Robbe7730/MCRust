use std::net::TcpStream;

use crate::structs::Chat;

use crate::packet_reader::PacketReader;
use crate::packets::ServerboundPacket;
use crate::packets::ClientboundPacket;

use crate::packets::legacy_ping_clientbound::LegacyPingClientboundPacket;
use crate::packets::status_response::StatusResponsePacket;
use crate::packets::pong::PongPacket;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
}

impl ConnectionState {
    pub fn from(i: isize) -> Result<Self, String> {
        match i {
            0 => Ok(ConnectionState::Handshaking),
            1 => Ok(ConnectionState::Status),
            2 => Ok(ConnectionState::Login),
            x => Err(format!("Invalid state {}", x))
        }
    }
}

pub struct ClientHandler {
    reader: PacketReader,
    state: ConnectionState,
}

impl ClientHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            reader: PacketReader::new(stream),
            state: ConnectionState::Handshaking,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("RUN!");
        loop {
            let packet = self.reader.read_packet(self.state)?;
            self.handle_packet(packet)?;
        }
    }

    fn handle_packet(&mut self, packet: ServerboundPacket) -> Result<(), String> {
        Ok(match packet {
            ServerboundPacket::LegacyPing(packet) => {
                self.send_packet(ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                    protocol_version: 127,
                    minecraft_version: "1.14.4".to_string(),
                    motd: "Hello from Rust!".to_string(),
                    curr_player_count: 13,
                    max_player_count: 37,
                }))?;
                println!("Legacy Ping @ {}:{}", packet.hostname, packet.port);
            },
            ServerboundPacket::Handshaking(packet) => {
                println!("Handshake @ {}:{} version {} to state {:?}", 
                         packet.server_address, 
                         packet.server_port,
                         packet.protocol_version,
                         packet.next_state
                );
                self.state = packet.next_state;
            },
            ServerboundPacket::StatusRequest(_) => {
                println!("Status Request");
                self.send_packet(ClientboundPacket::StatusResponse(StatusResponsePacket {
                    version_name: format!("MCRust 0.1.0"),
                    version_protocol: 498,
                    players_max: 37,
                    players_curr: 13,
                    sample: vec![],
                    description: Chat::new(format!("Hello from Rust!")),
                }))?;
            },
            ServerboundPacket::Ping(packet) => {
                println!("Ping with payload {}", packet.payload);
                self.send_packet(ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload
                }))?;
            }
        })
    }

    fn send_packet(&mut self, packet: ClientboundPacket) -> Result<(), String> {
        self.reader.send_packet(packet)
    }
}
