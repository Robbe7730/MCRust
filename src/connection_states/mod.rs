mod handshaking;
mod status;

use crate::error_type::ErrorType;
use crate::packets::packet_reader::PacketReader;
use crate::packets::serverbound::ServerboundPacket;
use crate::Server;

use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

pub use handshaking::HandshakingState;
pub use status::StatusState;

trait ConnectionState {
    fn handle_packet(
        &self,
        packet: ServerboundPacket,
        stream: Arc<Mutex<TcpStream>>,
        server: Arc<Mutex<Server>>,
    ) -> Result<ConnectionStateTransition, ErrorType>;
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

pub struct ClientHandler;

impl ClientHandler {
    pub fn run(stream: TcpStream, server: Arc<Mutex<Server>>) {
        let mut state: Arc<Mutex<dyn ConnectionState>> = Arc::new(Mutex::new(HandshakingState {}));
        let mut state_tag = ConnectionStateTag::Handshaking;
        let stream_arc = Arc::new(Mutex::new(stream));
        let mut reader = PacketReader::new(stream_arc.clone());
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
            {
                let state_locked = state.lock().expect("Could not lock state");
                result = (*state_locked).handle_packet(
                    packet.unwrap(),
                    stream_arc.clone(),
                    server.clone(),
                );
            }
            state_tag = match result {
                Ok(transition) => match transition {
                    ConnectionStateTransition::Remain => state_tag,
                    ConnectionStateTransition::TransitionTo(new_tag) => {
                        state = match new_tag {
                            ConnectionStateTag::Handshaking => {
                                Arc::new(Mutex::new(HandshakingState {}))
                            }
                            ConnectionStateTag::Status => Arc::new(Mutex::new(StatusState {})),
                            ConnectionStateTag::Login => {
                                unimplemented!();
                            }
                            ConnectionStateTag::Play => {
                                unimplemented!();
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
