pub mod legacy_ping_serverbound;
pub mod legacy_ping_clientbound;

use crate::packets::legacy_ping_serverbound::LegacyPingServerboundPacket;
use crate::packets::legacy_ping_clientbound::LegacyPingClientboundPacket;

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
}

pub enum ClientboundPacket {
    LegacyPing(LegacyPingClientboundPacket),
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
        }
    }
}
