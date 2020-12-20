use crate::packets::Serverbound;

use crate::packet_reader::PacketReader;

pub struct StatusRequestPacket {}

impl Serverbound for StatusRequestPacket {
    fn from_reader(_reader: &mut PacketReader) -> Result<Self, String> {
        Ok(Self {})
    }
}
