use super::super::Clientbound;
use super::super::packet_writer::PacketWriter;

#[derive(Debug, Clone)]
pub struct KeepAlivePacket {
    pub id: i64,
}

impl Clientbound for KeepAlivePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x1F);
        writer.add_signed_long(self.id);
        writer
    }
}
