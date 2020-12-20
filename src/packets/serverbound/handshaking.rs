use crate::packets::Serverbound;

use crate::client_handler::ConnectionState;
use crate::packet_reader::PacketReader;
use crate::error_type::ErrorType;

pub struct HandshakingPacket {
    pub protocol_version: isize,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: ConnectionState,
}

impl Serverbound for HandshakingPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType> {
        let protocol_version = reader.read_varint()?;
        let server_address = reader.read_string()?;
        let server_port = reader.read_unsigned_short()?;
        let next_state = ConnectionState::from(reader.read_varint()?)?;
        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }
}
