use std::convert::TryInto;

use crate::server::Recipe;
use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct DeclareRecipiesPacket {
    pub recipies: Vec<Recipe>,
}

impl Clientbound for DeclareRecipiesPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x5A);

        writer.add_varint(self.recipies.len().try_into().unwrap());
        for recipe in self.recipies.iter() {
            recipe.write(&mut writer);
        }

        writer
    }
}
