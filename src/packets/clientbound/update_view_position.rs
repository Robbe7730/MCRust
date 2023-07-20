use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct UpdateViewPositionPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Clientbound for UpdateViewPositionPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x40);

        writer.add_varint(self.chunk_x);
        writer.add_varint(self.chunk_z);

        writer
    }
}
