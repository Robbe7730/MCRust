use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use crate::error_type::ErrorType;
use crate::packet_reader::PacketReader;
use crate::packets::clientbound::*;
use crate::packets::serverbound::ServerboundPacket;
use crate::structs::Chat;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking = 0,
    Status = 1,
    Login = 2,
}

impl ConnectionState {
    pub fn from(i: isize) -> Result<Self, ErrorType> {
        match i {
            0 => Ok(ConnectionState::Handshaking),
            1 => Ok(ConnectionState::Status),
            2 => Ok(ConnectionState::Login),
            x => Err(ErrorType::Fatal(format!("Invalid state {}", x))),
        }
    }
}

pub struct ClientHandler {
    stream: Arc<Mutex<TcpStream>>,
    state: ConnectionState,
    reader: PacketReader,
}

impl ClientHandler {
    pub fn new(stream: TcpStream) -> Self {
        let stream = Arc::new(Mutex::new(stream));
        Self {
            stream: stream.clone(),
            state: ConnectionState::Handshaking,
            reader: PacketReader::new(stream.clone()),
        }
    }

    pub fn run(&mut self) {
        println!("RUN!");
        loop {
            let packet_result = self
                .reader
                .read_packet(self.state)
                .and_then(|packet| self.handle_packet(packet));
            match packet_result {
                Ok(_) => println!("Another succesful packet handled"),
                Err(ErrorType::Fatal(msg)) => {
                    println!("FATAL: {}", msg);
                    self.graceful_exit();
                    break;
                }
                Err(ErrorType::Recoverable(msg)) => {
                    println!("Whoops: {}", msg);
                }
                Err(ErrorType::GracefulExit) => {
                    self.graceful_exit();
                    break;
                }
            }
        }
    }

    fn graceful_exit(&mut self) {
        // The connection is closed when it goes out of scope, so for now nothing needs to be done
        // here
        println!("Going down");
    }

    pub fn send_packet(&mut self, packet: ClientboundPacket) -> Result<(), ErrorType> {
        packet.writer().write(self.stream.clone())
    }

    fn handle_packet(&mut self, packet: ServerboundPacket) -> Result<(), ErrorType> {
        Ok(match packet {
            ServerboundPacket::LegacyPing(packet) => {
                println!("Legacy Ping @ {}:{}", packet.hostname, packet.port);
                //TODO: un-hardcode this data
                self.send_packet(ClientboundPacket::LegacyPing(LegacyPingClientboundPacket {
                    protocol_version: 127,
                    minecraft_version: "1.14.4".to_string(),
                    motd: "Hello from Rust!".to_string(),
                    curr_player_count: 13,
                    max_player_count: 37,
                }))?;
                Err(ErrorType::GracefulExit)?
            }
            ServerboundPacket::Handshaking(packet) => {
                println!(
                    "Handshake @ {}:{} version {} to state {:?}",
                    packet.server_address,
                    packet.server_port,
                    packet.protocol_version,
                    packet.next_state
                );
                self.state = packet.next_state;
            }
            ServerboundPacket::StatusRequest(_) => {
                println!("Status Request");
                //TODO: un-hardcode this data
                self.send_packet(ClientboundPacket::StatusResponse(StatusResponsePacket {
                    version_name: format!("MCRust 0.1.0"),
                    version_protocol: 498,
                    players_max: 37,
                    players_curr: 13,
                    sample: vec![],
                    description: Chat::new(format!("Hello from Rust!")),
                }))?;
            }
            ServerboundPacket::Ping(packet) => {
                println!("Ping with payload {}", packet.payload);
                self.send_packet(ClientboundPacket::Pong(PongPacket {
                    payload: packet.payload,
                }))?;
                Err(ErrorType::GracefulExit)?
            }
        })
    }
}
