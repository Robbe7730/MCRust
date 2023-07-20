use crate::packets::packet_reader::PacketReader;
use crate::error_type::ErrorType;

use super::Serverbound;

#[derive(Debug)]
pub struct TeleportConfirmPacket {
    pub teleport_id: isize
}

impl Serverbound for TeleportConfirmPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            teleport_id: reader.read_varint()?,
        })
    }
}
