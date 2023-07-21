mod chunk_section;
mod chunk_column;

pub use chunk_section::ChunkSection;
pub use chunk_column::ChunkColumn;

use std::{sync::{Arc, RwLock}, collections::HashMap};

use rand::random;

use crate::{Eid, error_type::ErrorType, server::Entity};

#[derive(Clone)]
pub struct World {
    pub name: String,
    pub seed: [u8; 32],
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub entities: Arc<RwLock<HashMap<u32, Arc<RwLock<Entity>>>>>,
}

impl World {
    pub fn dummy() -> Self {
        Self {
            name: "overworld".to_string(),
            seed: [0; 32],
            reduced_debug_info: false,
            enable_respawn_screen: true,
            is_debug: false,
            is_flat: true,
            entities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // x, y, z are chunk indices (= position // 16)
    pub fn get_chunk_section(&self, _x: isize, _y: isize, _z: isize) -> ChunkSection {
        // TODO: actually implement this
        let chunk = [0u16; 4096];

        return ChunkSection::from(chunk);
    }

    pub fn get_chunk_column(&self, x: isize, z: isize) -> ChunkColumn {
        let mut ret = vec![];
        for y in 0..16 {
            ret.push(self.get_chunk_section(x, y, z));
        }
        return ret.into();
    }

    pub fn get_entity(&self, eid: Eid) -> Result<Option<Arc<RwLock<Entity>>>, ErrorType> {
        Ok(self
            .entities
            .read()
            .map_err(|e| {
                ErrorType::Fatal(format!(
                    "Could not lock entities for reading: {}",
                    e.to_string()
                ))
            })?
            .get(&eid)
            .cloned())
    }

    pub fn register_entity(&self, entity: Entity) -> Result<u32, ErrorType> {
        let mut eid: u32 = random();
        while self
            .entities
            .read()
            .map_err(|e| {
                ErrorType::Fatal(format!(
                    "Could not lock entities for reading: {}",
                    e.to_string()
                ))
            })?
            .contains_key(&eid)
        {
            eid = random();
        }
        self.entities
            .write()
            .map_err(|e| {
                ErrorType::Fatal(format!(
                    "Could not lock entities for writing: {}",
                    e.to_string()
                ))
            })?
            .insert(eid, Arc::new(RwLock::new(entity)));
        Ok(eid)
    }

}
