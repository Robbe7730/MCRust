use crate::world::Difficulty;
use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct ChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub difficulty_locked: bool,
}

impl Clientbound for ChangeDifficultyPacket {
    fn writer(&self) -> PacketWriter {
        // This changed to 0x0C in newer versions
        let mut writer = PacketWriter::new(0x0D);

        writer.add_unsigned_byte(self.difficulty.into());
        writer.add_boolean(self.difficulty_locked);

        writer
    }
}
