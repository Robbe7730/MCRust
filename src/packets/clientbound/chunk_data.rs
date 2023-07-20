use std::convert::TryInto;
use std::fmt::Debug;

use super::Clientbound;
use crate::packets::packet_writer::PacketWriter;
use crate::world::ChunkColumn;
use crate::nbt::{NBTTag, NamedNBTTag};

#[derive(Clone)]
pub struct ChunkDataPacket {
    x: i32,
    z: i32,
    full_chunk: bool,
    primary_bitmask: i32,
    heightmaps: NBTTag,
    biomes: Vec<i32>,
    data: Vec<u8>,
    block_entities: Vec<NBTTag>,
}

impl Debug for ChunkDataPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"x", &format!("{}", self.x))
            .entry(&"z", &format!("{}", self.z))
            .entry(&"full_chunk", &format!("{}", self.full_chunk))
            .entry(&"primary_bitmask", &format!("{:016b}", self.primary_bitmask))
            .entry(&"heightmaps", &"<stripped>")
            .entry(&"biomes", &"<stripped>")
            .entry(&"data", &"<stripped>")
            .entry(&"block_entities", &"<stripped>")
            .finish()
    }
}

impl Clientbound for ChunkDataPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x20);
        writer.add_signed_int(self.x);
        writer.add_signed_int(self.z);
        writer.add_boolean(self.full_chunk);
        writer.add_varint(self.primary_bitmask);
        writer.add_named_nbt(&NamedNBTTag::new("", self.heightmaps.clone()));

        if self.full_chunk {
            if self.biomes.len() != 1024 {
                eprintln!("INVALID BIOME DATA: expected 1024 biome entries, but got {}.", self.biomes.len())
            }
            writer.add_varint(self.biomes.len().try_into().unwrap());
            for biome in self.biomes.iter() {
                writer.add_varint(*biome);
            }
        }

        writer.add_varint(self.data.len().try_into().expect("Too much data for VarInt"));
        for byte in self.data.iter() {
            writer.add_unsigned_byte(*byte);
        }

        writer.add_varint(self.block_entities.len().try_into().expect("Too much data for VarInt"));
        for tag in self.block_entities.iter() {
            writer.add_nbt(tag);
        }

        writer
    }
}

impl ChunkDataPacket {
    pub fn from_chunk_column(
        x: i32,
        z: i32,
        column: ChunkColumn
    ) -> Self {
        let mut primary_bitmask = 0;
        let mut sections = vec![];
        for (i, maybe_section) in column.get_sections().iter().enumerate() {
            if let Some(section) = maybe_section {
                primary_bitmask |= 1 << i;
                sections.push(section);
            }
        }
        
        Self {
            x,
            z,
            full_chunk: true,
            primary_bitmask,
            heightmaps: column.get_heightmaps(),
            biomes: column.get_biomes(),
            data: column.to_packet_data(),
            block_entities: column.get_block_entities(),
        }
    }
}
