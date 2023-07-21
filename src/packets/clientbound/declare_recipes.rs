use std::convert::TryInto;

use crate::server::Recipe;
use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct DeclareRecipesPacket {
    pub recipes: Vec<Recipe>,
}

impl Clientbound for DeclareRecipesPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x5A);

        writer.add_varint(self.recipes.len().try_into().unwrap());
        for recipe in self.recipes.iter() {
            recipe.write(&mut writer);
        }

        writer
    }
}
