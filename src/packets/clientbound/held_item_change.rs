use super::super::Clientbound;
use super::super::packet_writer::PacketWriter;

pub struct HeldItemChangePacket {
    pub slot: u8,
}

impl Clientbound for HeldItemChangePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x3F);
        writer.add_unsigned_byte(self.slot);
        writer
    }
}
