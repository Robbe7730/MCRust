use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;

#[derive(Debug)]
pub struct ChatMessagePacket {
    pub message: String,
}

impl Serverbound for ChatMessagePacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {
            message: reader.read_string()?,
        })
    }
}
