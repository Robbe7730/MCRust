mod biome;
mod dimension;
mod dimension_codec;
mod entity;
mod player;
mod server_settings;
mod world;

pub use biome::*;
pub use dimension::*;
pub use dimension_codec::*;
pub use entity::*;
pub use player::*;
pub use server_settings::*;
pub use world::*;

use crate::error_type::ErrorType;
use crate::Eid;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use rand::random;
use uuid::Uuid;

pub struct Server {
    pub settings: ServerSettings,
    pub entities: Arc<RwLock<HashMap<u32, Arc<RwLock<Entity>>>>>,
    pub player_eids: Arc<RwLock<HashMap<Uuid, u32>>>,
    pub dimension_codec: DimensionCodec,
}

impl Server {
    pub fn new() -> Self {
        let mut dimension_codec = DimensionCodec::new();

        let only_dimension = Dimension::dummy();
        dimension_codec.add_dimension(only_dimension);

        let only_biome = Biome::dummy();
        dimension_codec.add_biome(only_biome);
        Server {
            settings: ServerSettings::dummy(),
            entities: Arc::new(RwLock::new(HashMap::new())),
            player_eids: Arc::new(RwLock::new(HashMap::new())),
            dimension_codec,
        }
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

    pub fn load_or_create_player(&self, username: &String, uuid: Uuid) -> Result<Eid, ErrorType> {
        // TODO: persistent player storage
        let player = Player::new(
            uuid,
            username.to_string(),
            self.settings.default_gamemode.clone(),
            self.dimension_codec.dimensions["mcrust:the_only_dimension"].clone(),
        );
        let eid = self.register_entity(Entity::PlayerEntity(player))?;
        self.player_eids
            .write()
            .map_err(|e| {
                ErrorType::Fatal(format!(
                    "Could not lock player eid mapping: {}",
                    e.to_string()
                ))
            })?
            .insert(uuid, eid);
        Ok(eid)
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
}
