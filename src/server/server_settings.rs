use super::World;

use crate::util::Gamemode;

use std::collections::HashMap;

#[derive(Clone)]
pub struct ServerSettings {
    pub version: String,
    pub protocol_version: usize,
    pub max_players: i32,
    pub motd: String,
    pub online: bool,
    pub is_hardcore: bool,
    pub default_gamemode: Gamemode,
    pub worlds: HashMap<String, World>,
    pub selected_world: String,
    pub view_distance: i32,
}

impl ServerSettings {
    pub fn dummy() -> Self {
        let mut worlds = HashMap::new();
        let only_world = World::dummy();
        let selected_world = only_world.name.clone();
        worlds.insert(only_world.name.clone(), only_world);

        Self {
            version: format!("MCRust 0.1.0"),
            protocol_version: 498,
            max_players: 20,
            motd: format!("Hello from Rust"),
            online: false,
            is_hardcore: false,
            default_gamemode: Gamemode::Creative,
            worlds,
            selected_world,
            view_distance: 8,
        }
    }
}
