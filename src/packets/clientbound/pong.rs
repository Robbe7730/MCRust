use crate::packets::Clientbound;

use crate::packet_writer::PacketWriter;

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
