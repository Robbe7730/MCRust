use crate::packets::Serverbound;
use crate::packets::packet_reader::PacketReader;
use crate::error_type::ErrorType;

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
