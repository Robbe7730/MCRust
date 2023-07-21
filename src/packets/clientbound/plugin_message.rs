use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct PluginMessagePacket {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Clientbound for PluginMessagePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x17);

        writer.add_string(&self.channel);
        self.data.iter().for_each(|b| writer.add_unsigned_byte(*b));

        writer
    }
}
