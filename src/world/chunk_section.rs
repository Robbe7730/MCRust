use std::convert::TryInto;

use crate::{packets::packet_writer::PacketWriter, nbt::NBTTag};

#[derive(Clone)]
pub struct ChunkSection {
    data: [u16; 4096]
}

impl From<[u16; 4096]> for ChunkSection {
    fn from(value: [u16; 4096]) -> Self {
        Self { data: value }
    }
}

impl ChunkSection {
    pub fn is_empty(&self) -> bool {
        return false;
    }

    pub fn to_packet_data(&self) -> Vec<u8> {
        let mut ret = vec![];

        ret.append(&mut self.num_blocks().to_be_bytes().into());
        ret.push(16); // bits per block CAUTION when changing this, don't forget to include
                      // padding!
        ret.append(&mut PacketWriter::to_varint((self.data.len() * 2).try_into().unwrap()));
        for block in self.data {
            ret.append(&mut block.to_be_bytes().into());
        }

        ret
    }

    pub fn get_biomes(&self) -> Vec<i32> {
        let ret = vec![0; 4*4*4];
        return ret;
    }

    pub fn get_block_entities(&self) -> Vec<NBTTag> {
        return vec![];
    }
    
    pub fn num_blocks(&self) -> i16 {
        return 1;
    }
}
