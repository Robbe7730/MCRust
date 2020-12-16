use crate::packets::Serverbound;

use crate::packet_reader::PacketReader;

pub struct PingPacket {
    pub payload: i64,
}

impl Serverbound for PingPacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, String> {
        Ok(Self {
            payload: reader.read_signed_long()?,
        })
    }
}
