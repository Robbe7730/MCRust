use std::collections::HashMap;

use crate::nbt::NBTTag;

use super::ChunkSection;

pub struct ChunkColumn {
    sections: Vec<ChunkSection>
}

impl From<Vec<ChunkSection>> for ChunkColumn {
    fn from(value: Vec<ChunkSection>) -> Self {
        Self {
            sections: value
        }
    }
}

impl ChunkColumn {
    pub fn get_heightmaps(&self) -> NBTTag {
        let heightmaps = HashMap::new();

        // TODO: heightmaps
        // heightmaps.insert("MOTION_BLOCKING", NBTTag::LongArray(vec![0; 16*16]));
        // heightmaps.insert("WORLD_SURFACE", NBTTag::LongArray(vec![0; 16*16]));

        heightmaps.into()
    }

    pub fn get_sections(&self) -> Vec<Option<&ChunkSection>> {
        let mut ret = vec![];
        for section in self.sections.iter() {
            if section.is_empty() {
                ret.push(None);
            } else {
                ret.push(Some(section));
            }
        }

        ret
    }

    pub fn get_biomes(&self) -> Vec<i32> {
        let mut ret = vec![];

        for maybe_section in self.get_sections() {
            if let Some(section) = maybe_section {
                ret.append(&mut section.get_biomes());
            }
        }

        ret
    }

    pub fn get_block_entities(&self) -> Vec<NBTTag> {
        let mut ret = vec![];

        for maybe_section in self.get_sections() {
            if let Some(section) = maybe_section {
                ret.append(&mut section.get_block_entities());
            }
        }

        ret
    }

    pub fn to_packet_data(&self) -> Vec<u8> {
        let mut ret = vec![];

        for maybe_section in self.get_sections() {
            if let Some(section) = maybe_section {
                ret.append(&mut section.to_packet_data());
            }
        }

        ret
    }
}
