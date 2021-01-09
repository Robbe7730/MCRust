use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;

#[derive(Debug)]
pub struct KeepAlivePacket {
    pub id: i64,
}

impl Serverbound for KeepAlivePacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            id: reader.read_signed_long()?,
        })
    }
}
