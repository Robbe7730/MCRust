use super::super::packet_reader::PacketReader;
use super::super::Serverbound;

use crate::error_type::ErrorType;

#[derive(Debug)]
pub struct StatusRequestPacket {}

impl Serverbound for StatusRequestPacket {
    fn from_reader(_reader: &mut PacketReader) -> Result<Self, ErrorType> {
        Ok(Self {})
    }
}
