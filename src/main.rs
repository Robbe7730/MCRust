use std::net::TcpListener;

mod chat;
mod client_handler;
mod error_type;
mod nbt;
mod packets;
mod util;

use client_handler::ClientHandler;
use error_type::ErrorType;
use nbt::NBTTag;
use nbt::NamedNBTTag;
use util::Gamemode;

use rand::random;
use uuid::Uuid;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;

// TODO: fix the ugly mess that this file is right now...
// TODO: Make Players, Biomes, Worlds, Entities in a repository-style storage
// TODO: Split up in different files

#[derive(Clone)]
pub struct Player {
    uuid: Uuid,
    username: String,
    gamemode: Gamemode,
    previous_gamemode: Option<Gamemode>,
    dimension: Dimension,
}

impl Player {
    pub fn new(uuid: Uuid, username: String, gamemode: Gamemode, dimension: Dimension) -> Self {
        Self {
            uuid,
            username,
            gamemode,
            previous_gamemode: None,
            dimension,
        }
    }
}

#[derive(Clone)]
pub enum Entity {
    PlayerEntity(Arc<Player>),
}

#[derive(Clone)]
pub struct World {
    seed: [u8; 32],
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    is_debug: bool,
    is_flat: bool,
}

#[derive(Clone)]
pub struct Dimension {
    id: i32,
    name: String,
    element: HashMap<String, NBTTag>,
}

impl Dimension {
    pub fn as_nbt(&self) -> NBTTag {
        NBTTag::Compound(vec![
            NamedNBTTag::new("name", NBTTag::String(self.name.clone())),
            NamedNBTTag::new("id", NBTTag::Int(self.id)),
            NamedNBTTag::new(
                "element",
                self.element_as_nbt()
            ),
        ])
    }

    pub fn element_as_nbt(&self) -> NBTTag {
        NBTTag::Compound(
            self.element
                .iter()
                .map(|(k, v)| NamedNBTTag::new(k, v.clone()))
                .collect(),
        )
    }

    pub fn set_float(&mut self, key: &str, value: f32) {
        self.element.insert(key.to_string(), NBTTag::Float(value));
    }

    pub fn set_byte(&mut self, key: &str, value: u8) {
        self.element.insert(key.to_string(), NBTTag::Byte(value));
    }

    pub fn set_int(&mut self, key: &str, value: i32) {
        self.element.insert(key.to_string(), NBTTag::Int(value));
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        self.element.insert(key.to_string(), NBTTag::String(value.to_string()));
    }
}

#[derive(Clone)]
pub struct Biome {
    id: i32,
    name: String,
    element: HashMap<String, NBTTag>,
}

impl Biome {
    pub fn as_nbt(&self) -> NBTTag {
        NBTTag::Compound(vec![
            NamedNBTTag::new("name", NBTTag::String(self.name.clone())),
            NamedNBTTag::new("id", NBTTag::Int(self.id)),
            NamedNBTTag::new(
                "element",
                NBTTag::Compound(
                    self.element
                        .iter()
                        .map(|(k, v)| NamedNBTTag::new(k, v.clone()))
                        .collect(),
                ),
            ),
        ])
    }

    pub fn set_float(&mut self, key: &str, value: f32) {
        self.element.insert(key.to_string(), NBTTag::Float(value));
    }

    pub fn set_byte(&mut self, key: &str, value: u8) {
        self.element.insert(key.to_string(), NBTTag::Byte(value));
    }

    pub fn set_int(&mut self, key: &str, value: i32) {
        self.element.insert(key.to_string(), NBTTag::Int(value));
    }

    pub fn set_string(&mut self, key: &str, value: &str) {
        self.element.insert(key.to_string(), NBTTag::String(value.to_string()));
    }

    pub fn set_compound(&mut self, key: &str, value: NBTTag) {
        self.element.insert(key.to_string(), value);
    }
}

#[derive(Clone)]
pub struct DimensionCodec {
    dimensions: HashMap<String, Dimension>,
    biomes: HashMap<String, Biome>,
}

impl DimensionCodec {
    pub fn new() -> Self {
        Self {
            dimensions: HashMap::new(),
            biomes: HashMap::new(),
        }
    }

    pub fn as_nbt(&self) -> NamedNBTTag {
        NamedNBTTag::new(
            "dimension_codec",
            NBTTag::Compound(vec![
                NamedNBTTag::new(
                    "minecraft:dimension_type",
                    NBTTag::Compound(vec![
                        NamedNBTTag::new(
                            "type",
                            NBTTag::String("minecraft:dimension_type".to_string()),
                        ),
                        NamedNBTTag::new(
                            "value",
                            NBTTag::List(self.dimensions.values().map(|d| d.as_nbt()).collect()),
                        ),
                    ]),
                ),
                NamedNBTTag::new(
                    "minecraft:worldgen/biome",
                    NBTTag::Compound(vec![
                        NamedNBTTag::new(
                            "type",
                            NBTTag::String("minecraft:worldgen/biome".to_string()),
                        ),
                        NamedNBTTag::new(
                            "value",
                            NBTTag::List(self.biomes.values().map(|d| d.as_nbt()).collect()),
                        ),
                    ]),
                )
            ]),
        )
    }

    pub fn add_dimension(&mut self, dim: Dimension) {
        self.dimensions.insert(dim.name.clone(), dim.clone());
    }

    pub fn add_biome(&mut self, biome: Biome) {
        self.biomes.insert(biome.name.clone(), biome.clone());
    }
}

#[derive(Clone)]
pub struct ServerSettings {
    version: String,
    protocol_version: usize,
    max_players: usize,
    motd: String,
    online: bool,
    is_hardcore: bool,
    default_gamemode: Gamemode,
    worlds: HashMap<String, World>,
    selected_world: String,
    view_distance: usize,
}

pub struct Server {
    settings: ServerSettings,
    entities: Arc<RwLock<HashMap<u32, Entity>>>,
    player_eids: Arc<RwLock<HashMap<Uuid, u32>>>,
    dimension_codec: DimensionCodec,
}

impl Server {
    pub fn new() -> Self {
        let mut worlds = HashMap::new();
        worlds.insert(
            "wereld".to_string(),
            World {
                seed: [0; 32],
                reduced_debug_info: false,
                enable_respawn_screen: true,
                is_debug: false,
                is_flat: true,
            },
        );

        let mut dimension_codec = DimensionCodec::new();

        // Implementing a dimension with the basic fields the Notchian client requires
        let mut only_dimension = Dimension {
            id: 0,
            name: "mcrust:the_only_dimension".to_string(),
            element: HashMap::new(),
        };
        // TODO: make these required fields in the Dimension struct
        only_dimension.set_float("ambient_light", 0.0);
        only_dimension.set_string("infiniburn", "minecraft:infiniburn_overworld");
        only_dimension.set_int("logical_height", 256);
        only_dimension.set_byte("has_raids", 0);
        only_dimension.set_byte("respawn_anchor_works", 0);
        only_dimension.set_byte("bed_works", 0);
        only_dimension.set_byte("piglin_safe", 0);
        only_dimension.set_float("coordinate_scale", 1.0);
        only_dimension.set_byte("natural", 0);
        only_dimension.set_byte("ultrawarm", 0);
        only_dimension.set_byte("has_ceiling", 0);
        only_dimension.set_byte("has_skylight", 0);
        dimension_codec.add_dimension(only_dimension);

        // Implementing a biome with the basic fields the Notchian client requires
        // Somehow, a "minecraft:plains" biome is required for the Notchian server to work
        let mut only_biome = Biome {
            id: 0,
            name: "minecraft:plains".to_string(),
            element: HashMap::new(),
        };
        only_biome.set_compound("effects", NBTTag::Compound(vec![
            NamedNBTTag::new("sky_color", NBTTag::Int(7907327)),
            NamedNBTTag::new("water_fog_color", NBTTag::Int(329011)),
            NamedNBTTag::new("water_color", NBTTag::Int(12638463)),
            NamedNBTTag::new("fog_color", NBTTag::Int(4159204)),
        ]));
        only_biome.set_float("scale", 0.05);
        only_biome.set_float("depth", 0.125);
        only_biome.set_string("category", "none");
        only_biome.set_string("precipitation", "rain");
        only_biome.set_float("downfall", 0.05);
        only_biome.set_float("temperature", 0.8);
        dimension_codec.add_biome(only_biome);
        Server {
            settings: ServerSettings {
                version: format!("MCRust 0.1.0"),
                protocol_version: 498,
                max_players: 20,
                motd: format!("Hello from Rust"),
                online: false,
                is_hardcore: false,
                default_gamemode: Gamemode::Spectator,
                worlds,
                selected_world: "wereld".to_string(),
                view_distance: 8,
            },
            entities: Arc::new(RwLock::new(HashMap::new())),
            player_eids: Arc::new(RwLock::new(HashMap::new())),
            dimension_codec,
        }
    }

    pub fn register_entity(&mut self, entity: &Entity) -> Result<u32, ErrorType> {
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
                    "Could nog lock entities for writing: {}",
                    e.to_string()
                ))
            })?
            .insert(eid, (*entity).clone());
        Ok(eid)
    }

    pub fn load_or_create_player(
        &mut self,
        username: &String,
        uuid: Uuid,
    ) -> Result<(Arc<Player>, u32), ErrorType> {
        // TODO: persistent player storage
        let player = Arc::new(Player::new(
            uuid,
            username.to_string(),
            self.settings.default_gamemode.clone(),
            self.dimension_codec.dimensions["mcrust:the_only_dimension"].clone(),
        ));
        let eid = self.register_entity(&Entity::PlayerEntity(player.clone()))?;
        self.player_eids
            .write()
            .map_err(|e| {
                ErrorType::Fatal(format!(
                    "Could not lock player eid mapping: {}",
                    e.to_string()
                ))
            })?
            .insert(uuid, eid);
        Ok((player, eid))
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Could not start server");

    let server = Arc::new(Mutex::new(Server::new()));

    for stream in listener.incoming() {
        let server_copy = server.clone();
        thread::spawn(|| {
            ClientHandler::new(stream.expect("Invalid stream"), server_copy).run();
        });
    }
}
