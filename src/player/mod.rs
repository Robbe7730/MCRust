mod abilities;
mod gamemode;

pub use abilities::*;
pub use gamemode::*;

use uuid::Uuid;

use crate::server::Dimension;

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

#[derive(Clone)]
pub struct Look {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone)]
pub struct RecipeBookState {
    pub crafting_table_open: bool,
    pub crafting_table_filter: bool,
    pub furnace_open: bool,
    pub furnace_filter: bool,
    pub blast_furnace_open: bool,
    pub blast_furnace_filter: bool,
    pub smoker_open: bool,
    pub smoker_filter: bool,
}

#[derive(Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub gamemode: Gamemode,
    pub previous_gamemode: Option<Gamemode>,
    pub dimension: Dimension,
    pub selected_slot: u8,
    pub position: Position,
    pub look: Look,
    pub abilities: Abilities,
    pub flying_speed: f32,
    pub fov_modifier: f32,
    pub recipe_book_state: RecipeBookState,
    pub unlocked_recipies: Vec<String>,
}

impl Player {
    pub fn new(uuid: Uuid, username: String, gamemode: Gamemode, dimension: Dimension) -> Self {
        Self {
            uuid,
            username,
            gamemode,
            previous_gamemode: None,
            dimension,
            selected_slot: 0,
            position: Position { x: 10.0, y: 64.0, z: 20.0, on_ground: true },
            look: Look {
                yaw: 0.0,
                pitch: 0.0,
            },
            abilities: Abilities::new()
                .add_ability(Ability::Invulnerable)
                .add_ability(Ability::Flying)
                .add_ability(Ability::AllowFlying)
                .add_ability(Ability::CreativeMode),
            flying_speed: 0.05,
            fov_modifier: 0.1,
            recipe_book_state: RecipeBookState {
                crafting_table_open: false,
                crafting_table_filter: false,
                furnace_open: false,
                furnace_filter: false,
                blast_furnace_open: false,
                blast_furnace_filter: false,
                smoker_open: false,
                smoker_filter: false
            },
            unlocked_recipies: vec![],
        }
    }

    pub fn offline_player_uuid(username: &String) -> Uuid {
        let username_bytes = format!("OfflinePlayer:{}", username)
            .bytes()
            .collect::<Vec<u8>>();
        Uuid::new_v3(&Uuid::NAMESPACE_URL, &username_bytes)
    }
}
