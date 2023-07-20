use crate::error_type::ErrorType;
use super::super::Serverbound;
use super::super::packet_reader::PacketReader;

#[derive(Debug)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Serverbound for PluginMessagePacket {
    fn from_reader(reader: &mut PacketReader) -> Result<Self, ErrorType>{
        let channel = reader.read_string()?;
        let data = reader.read_until_end()?;
        Ok(Self {
            channel,
            data
        })
    }
}
