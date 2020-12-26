pub mod join_game;
pub mod legacy_ping;
pub mod login_success;
pub mod pong;
pub mod status_response;

pub use join_game::*;
pub use legacy_ping::*;
pub use login_success::*;
pub use pong::*;
pub use status_response::*;

use crate::packets::packet_writer::PacketWriter;

pub enum ClientboundPacket {
    LegacyPing(LegacyPingClientboundPacket),
    StatusResponse(StatusResponsePacket),
    Pong(PongPacket),
    LoginSuccess(LoginSuccessPacket),
    JoinGame(JoinGamePacket),
}

pub trait Clientbound {
    fn writer(&self) -> PacketWriter;
}

impl Clientbound for ClientboundPacket {
    fn writer(&self) -> PacketWriter {
        match self {
            ClientboundPacket::LegacyPing(p) => p.writer(),
            ClientboundPacket::StatusResponse(p) => p.writer(),
            ClientboundPacket::Pong(p) => p.writer(),
            ClientboundPacket::LoginSuccess(p) => p.writer(),
            ClientboundPacket::JoinGame(p) => p.writer(),
        }
    }
}
