use super::super::Serverbound;
use super::super::packet_reader::PacketReader;

use crate::client_handler::ConnectionStateTag;
use crate::error_type::ErrorType;

#[derive(Debug)]
pub struct HandshakingPacket {
    pub protocol_version: isize,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ConnectionStateTag,
}

impl Serverbound for HandshakingPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        let protocol_version = reader.read_varint()?;
        let server_address = reader.read_string()?;
        let server_port = reader.read_unsigned_short()?;
        let next_state = ConnectionStateTag::from(reader.read_varint()?)?;
        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
