pub mod legacy_ping;
pub mod pong;
pub mod status_response;
pub mod login_success;

pub use legacy_ping::*;
pub use pong::*;
pub use status_response::*;
pub use login_success::*;

use crate::packet_writer::PacketWriter;

pub enum ClientboundPacket {
    LegacyPing(LegacyPingClientboundPacket),
    StatusResponse(StatusResponsePacket),
    Pong(PongPacket),
    LoginSuccess(LoginSuccessPacket),
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
        }
    }
}
