use super::super::Clientbound;
use super::super::packet_writer::PacketWriter;

#[derive(Debug)]
pub struct PongPacket {
    pub payload: i64,
}

impl Clientbound for PongPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x01);
        writer.add_signed_long(self.payload);
        writer
    }
}
