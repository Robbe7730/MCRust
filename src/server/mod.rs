mod biome;
mod dimension;
mod dimension_codec;
mod entity;
mod server_settings;
mod recipe;
mod command;
mod command_parser;

pub use biome::*;
pub use dimension::*;
pub use dimension_codec::*;
pub use entity::*;
pub use server_settings::*;
pub use recipe::*;
pub use command::*;

use crate::error_type::ErrorType;
use crate::nbt::NBTTag;
use crate::packets::packet_writer::PacketWriter;
use crate::player::OPLevel;
use crate::player::Player;
use crate::world::World;

use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use std::sync::RwLock;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub entries: Vec<i32>,
}

impl Tag {
    pub fn write(&self, writer: &mut PacketWriter) {
        writer.add_string(&self.name);
        writer.add_varint(self.entries.len().try_into().unwrap());
        for entry in self.entries.iter() {
            writer.add_varint(*entry);
        }
    }
}

pub struct Tags {
    pub block_tags: Vec<Tag>,
    pub item_tags: Vec<Tag>,
    pub fluid_tags: Vec<Tag>,
    pub entity_tags: Vec<Tag>,
}

pub struct ServerData {
    pub settings: ServerSettings,
    pub player_eids: Arc<RwLock<HashMap<Uuid, i32>>>,
    pub dimension_codec: DimensionCodec,
    pub recipes: Vec<Recipe>,
    pub tags: Tags,
    pub command_nodes: Vec<CommandNode>,
    pub command_root_node: i32,
}

impl ServerData {
    pub fn new() -> Self {
        let mut dimension_codec = DimensionCodec::new();

        let only_dimension = Dimension::dummy();
        dimension_codec.add_dimension(only_dimension);

        let only_biome = Biome::dummy();
        dimension_codec.add_biome(only_biome);

        let (command_nodes, command_root_node) = Self::load_commands();

        Self {
            settings: ServerSettings::dummy(),
            player_eids: Arc::new(RwLock::new(HashMap::new())),
            dimension_codec,
            recipes: Self::load_recipes(),
            tags: Self::load_tags(),
            command_nodes,
            command_root_node,
        }
    }

    pub fn load_or_create_player(&self, username: &String, uuid: Uuid) -> Result<i32, ErrorType> {
        // TODO: persistent player storage
        let player = Player::new(
            uuid,
            username.to_string(),
            self.settings.default_gamemode.clone(),
            self.dimension_codec.dimensions["mcrust:the_only_dimension"].clone(),
            OPLevel::Owner,
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

    pub fn load_recipes() -> Vec<Recipe> {
        // TODO implement this
        return vec![
            Recipe {
                id: "minecraft:dirt_shapeless".to_string(),
                data: RecipeData::CraftingShapeless(
                    "test1".to_string(),
                    vec![
                        vec![
                            Slot {
                                present: true,
                                item_id: Some(9),
                                count: Some(1),
                                nbt: Some(NBTTag::End)
                            }
                        ]
                    ], 
                    Slot {
                        present: true,
                        item_id: Some(9),
                        count: Some(2),
                        nbt: Some(NBTTag::End)
                    }
                )
            },
            Recipe {
                id: "minecraft:dirt_shaped".to_string(),
                data: RecipeData::CraftingShaped(
                    2,
                    2,
                    "test2".to_string(),
                    vec![
                        vec![
                            Slot {
                                present: true,
                                item_id: Some(9),
                                count: Some(1),
                                nbt: Some(NBTTag::End)
                            },
                        ], vec![
                            Slot {
                                present: true,
                                item_id: Some(9),
                                count: Some(1),
                                nbt: Some(NBTTag::End)
                            },
                        ], vec![
                            Slot {
                                present: true,
                                item_id: Some(9),
                                count: Some(1),
                                nbt: Some(NBTTag::End)
                            },
                        ], vec![
                            Slot {
                                present: true,
                                item_id: Some(9),
                                count: Some(1),
                                nbt: Some(NBTTag::End)
                            }
                        ]
                    ], 
                    Slot {
                        present: true,
                        item_id: Some(9),
                        count: Some(9),
                        nbt: Some(NBTTag::End)
                    }
                )
            }
        ]
    }

    fn load_tags() -> Tags {
        Tags {
            block_tags: vec![],
            item_tags: vec![],
            fluid_tags: vec![],
            entity_tags: vec![]
        }
    }

    fn load_commands() -> (Vec<CommandNode>, i32) {
        return (
            vec![
                CommandNode::Root(false, vec![1, 2], None),
                CommandNode::Literal(true, vec![], None, "test".to_string()),
                CommandNode::Literal(false, vec![3], None, "command".to_string()),
                CommandNode::Argument(true, vec![4], None, "argument".to_string(), CommandParserType::MinecraftColor(), None),
                CommandNode::Argument(true, vec![], None, "argument2".to_string(), CommandParserType::BrigadierInteger(Some(0), Some(10)), None)
            ],
            0
        );
    }
}
