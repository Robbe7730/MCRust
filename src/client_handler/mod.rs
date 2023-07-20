mod handshaking;
mod login;
mod play;
mod status;

use crate::error_type::ErrorType;
use crate::packets::clientbound::Clientbound;
use crate::packets::clientbound::ClientboundPacket;
use crate::packets::packet_reader::PacketReader;
use crate::packets::serverbound::ServerboundPacket;
use crate::Server;

use std::net::TcpStream;
use std::sync::Arc;

pub use handshaking::HandshakingState;
pub use login::LoginState;
pub use play::PlayState;
pub use status::StatusState;

#[derive(Debug, PartialEq)]
pub enum ConnectionState {
    Handshaking(HandshakingState),
    Status(StatusState),
    Login(LoginState),
    Play(PlayState),
}

impl ConnectionState {
    pub fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: TcpStream,
        server: Arc<Server>,
    ) -> Result<ConnectionStateTransition, ErrorType> {
        match self {
            ConnectionState::Handshaking(s) => s.handle_packet(packet, stream, server),
            ConnectionState::Status(s) => s.handle_packet(packet, stream, server),
            ConnectionState::Login(s) => s.handle_packet(packet, stream, server),
            ConnectionState::Play(s) => s.handle_packet(packet, stream, server),
        }
    }
}

trait ConnectionStateTrait {
    fn handle_packet(
        &mut self,
        packet: ServerboundPacket,
        stream: TcpStream,
        server: Arc<Server>,
    ) -> Result<ConnectionStateTransition, ErrorType>;

    fn from_state(prev_state: ConnectionState) -> Result<Self, ErrorType>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum ConnectionStateTag {
    Handshaking,
    Status,
    Login,
    Play,
    Exit,
}

#[derive(Debug)]
pub enum ConnectionStateTransition {
    Remain,
    TransitionTo(ConnectionStateTag),
}

impl ConnectionStateTag {
    pub fn from(i: isize) -> Result<Self, ErrorType> {
        match i {
            0 => Ok(Self::Handshaking),
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Play),
            x => Err(ErrorType::Fatal(format!("Invalid connection state {}", x))),
        }
    }
}

pub struct ClientHandler {
    stream: TcpStream,
    server: Arc<Server>,
}

impl ClientHandler {
    pub fn new(stream: TcpStream, server: Arc<Server>) -> ClientHandler {
        Self {
            stream,
            server,
        }
    }

    pub fn send_packet(&self, packet: ClientboundPacket) -> Result<(), ErrorType> {
        packet.writer().write(
            self.stream
                .try_clone()
                .map_err(|e| ErrorType::Fatal(format!("Could not clone TCP stream: {:?}", e)))?,
        )
    }

    pub fn run(&self) {
        let mut state: ConnectionState = ConnectionState::Handshaking(HandshakingState {});
        let mut state_tag = ConnectionStateTag::Handshaking;
        let mut reader = PacketReader::new(
            self.stream.try_clone().expect("Could not clone TCP stream"),
            0
        );
        while state_tag != ConnectionStateTag::Exit {
            let packet = reader.read_packet(&state_tag);
            if packet.is_err() {
                match packet {
                    Err(ErrorType::Fatal(msg)) => {
                        println!("FATAL: {}", msg);
                        break;
                    }
                    Err(ErrorType::Recoverable(msg)) => {
                        println!("Whoops: {}", msg);
                        continue;
                    }
                    Err(ErrorType::GracefulExit) => {
                        println!("Goodbye o/");
                        break;
                    }
                    Ok(_) => unreachable!(),
                }
            }
            let result;
            result = state.handle_packet(
                packet.unwrap(),
                self.stream.try_clone().expect("Could not lock TCP stream"),
                self.server.clone(),
            );
            state_tag = match result {
                Ok(transition) => match transition {
                    ConnectionStateTransition::Remain => state_tag,
                    ConnectionStateTransition::TransitionTo(new_tag) => {
                        state = match new_tag {
                            ConnectionStateTag::Handshaking => ConnectionState::Handshaking(
                                HandshakingState::from_state(state).unwrap(),
                            ),
                            ConnectionStateTag::Status => {
                                ConnectionState::Status(StatusState::from_state(state).unwrap())
                            }
                            ConnectionStateTag::Login => {
                                ConnectionState::Login(LoginState::from_state(state).unwrap())
                            }
                            ConnectionStateTag::Play => {
                                ConnectionState::Play(PlayState::from_state(state).unwrap())
                            }
                            ConnectionStateTag::Exit => {
                                break;
                            }
                        };
                        new_tag
                    }
                },
                Err(ErrorType::Fatal(msg)) => {
                    println!("FATAL: {}", msg);
                    ConnectionStateTag::Exit
                }
                Err(ErrorType::Recoverable(msg)) => {
                    println!("Whoops: {}", msg);
                    state_tag
                }
                Err(ErrorType::GracefulExit) => {
                    println!("Goodbye o/");
                    ConnectionStateTag::Exit
                }
            }
        }
    }
}
