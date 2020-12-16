pub mod legacy_ping_serverbound;
pub mod legacy_ping_clientbound;
pub mod handshaking;
pub mod status_request;
pub mod status_response;
pub mod ping;
pub mod pong;

use crate::packets::legacy_ping_serverbound::LegacyPingServerboundPacket;
use crate::packets::legacy_ping_clientbound::LegacyPingClientboundPacket;
use crate::packets::handshaking::HandshakingPacket;
use crate::packets::status_request::StatusRequestPacket;
use crate::packets::status_response::StatusResponsePacket;
use crate::packets::ping::PingPacket;
use crate::packets::pong::PongPacket;

use crate::packet_reader::PacketReader;
use crate::packet_writer::PacketWriter;

#[macro_export]
macro_rules! expect_equal {
    ( $actual:expr, $expected:expr ) => {
        {
            if ($actual != $expected) {
                Err(format!("Expedted {} but got {}", $expected, $actual))?
            } else {
                $expected
            }
        }
    };
}

pub enum ServerboundPacket {
    LegacyPing(LegacyPingServerboundPacket),
    Handshaking(HandshakingPacket),
    StatusRequest(StatusRequestPacket),
    Ping(PingPacket),
}

pub enum ClientboundPacket {
    LegacyPing(LegacyPingClientboundPacket),
    StatusResponse(StatusResponsePacket),
    Pong(PongPacket),
}

pub trait Serverbound {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, String>
        where Self : Sized;
}

pub trait Clientbound {
    fn writer(&self) -> PacketWriter;
}

impl Clientbound for ClientboundPacket {
    fn writer(&self) -> PacketWriter{
        match self {
            ClientboundPacket::LegacyPing(p) => p.writer(),
            ClientboundPacket::StatusResponse(p) => p.writer(),
            ClientboundPacket::Pong(p) => p.writer(),
        }
    }
}
