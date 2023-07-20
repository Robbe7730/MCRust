mod biome;
mod dimension;
mod dimension_codec;
mod entity;
mod player;
mod server_settings;

pub use biome::*;
pub use dimension::*;
pub use dimension_codec::*;
pub use entity::*;
pub use player::*;
pub use server_settings::*;

use crate::error_type::ErrorType;
use crate::Eid;
use crate::world::World;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use uuid::Uuid;

pub struct ServerData {
    pub settings: ServerSettings,
    pub player_eids: Arc<RwLock<HashMap<Uuid, u32>>>,
    pub dimension_codec: DimensionCodec,
}

impl ServerData {
    pub fn new() -> Self {
        let mut dimension_codec = DimensionCodec::new();

        let only_dimension = Dimension::dummy();
        dimension_codec.add_dimension(only_dimension);

        let only_biome = Biome::dummy();
        dimension_codec.add_biome(only_biome);
        Self {
            settings: ServerSettings::dummy(),
            player_eids: Arc::new(RwLock::new(HashMap::new())),
            dimension_codec,
        }
    }

    pub fn load_or_create_player(&self, username: &String, uuid: Uuid) -> Result<Eid, ErrorType> {
        // TODO: persistent player storage
        let player = Player::new(
            uuid,
            username.to_string(),
            self.settings.default_gamemode.clone(),
            self.dimension_codec.dimensions["mcrust:the_only_dimension"].clone(),
        );
        let world: &World = self
            .settings
            .worlds
            .get(&self.settings.selected_world)
            .ok_or(ErrorType::Fatal("Invalid selected".to_string()))?;
        let eid = world.register_entity(Entity::PlayerEntity(player))?;
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
}
