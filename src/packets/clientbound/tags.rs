use std::convert::TryInto;

use crate::packets::packet_writer::PacketWriter;
use crate::server::{Tag, Tags};

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct TagsPacket {
    pub block_tags: Vec<Tag>,
    pub item_tags: Vec<Tag>,
    pub fluid_tags: Vec<Tag>,
    pub entity_tags: Vec<Tag>,
}

impl Clientbound for TagsPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x5B);

        writer.add_varint(self.block_tags.len().try_into().unwrap());
        for tag in self.block_tags.iter() {
            tag.write(&mut writer);
        }

        writer.add_varint(self.item_tags.len().try_into().unwrap());
        for tag in self.item_tags.iter() {
            tag.write(&mut writer);
        }

        writer.add_varint(self.fluid_tags.len().try_into().unwrap());
        for tag in self.fluid_tags.iter() {
            tag.write(&mut writer);
        }

        writer.add_varint(self.entity_tags.len().try_into().unwrap());
        for tag in self.entity_tags.iter() {
            tag.write(&mut writer);
        }

        writer
    }
}

impl TagsPacket {
    pub fn from_tags(tags: &Tags) -> Self {
        Self {
            block_tags: tags.block_tags.clone(),
            item_tags: tags.item_tags.clone(),
            fluid_tags: tags.fluid_tags.clone(),
            entity_tags: tags.entity_tags.clone()
        }
    }
}
