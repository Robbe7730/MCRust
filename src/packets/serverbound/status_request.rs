use crate::packets::Serverbound;
use crate::packets::packet_reader::PacketReader;
use crate::error_type::ErrorType;

pub struct StatusRequestPacket {}

impl Serverbound for StatusRequestPacket {
    fn from_reader(_reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {})
    }
}
