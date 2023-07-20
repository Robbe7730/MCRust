use super::Serverbound;
use crate::packets::packet_reader::PacketReader;
use crate::error_type::ErrorType;

#[derive(Debug)]
pub struct HeldItemChangePacket {
    pub slot: i16,
}

impl Serverbound for HeldItemChangePacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(HeldItemChangePacket{
            slot: reader.read_signed_short()?
        })
    }
}
