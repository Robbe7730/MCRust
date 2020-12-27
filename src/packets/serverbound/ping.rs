use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;

pub struct PingPacket {
    pub payload: i64,
}

impl Serverbound for PingPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            payload: reader.read_signed_long()?,
        })
    }
}
