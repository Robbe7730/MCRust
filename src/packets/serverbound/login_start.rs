use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;

#[derive(Debug)]
pub struct LoginStartPacket {
    pub username: String,
}

impl Serverbound for LoginStartPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            username: reader.read_string()?,
        })
    }
}
