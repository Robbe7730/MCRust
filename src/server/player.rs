use super::Dimension;

use crate::util::Gamemode;

use uuid::Uuid;

#[derive(Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Clone)]
pub struct Look {
    pub yaw: f32,
    pub pitch: f32,
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
            position: Position { x: 0, y: 0, z: 0 },
            look: Look {
                yaw: 0.0,
                pitch: 0.0,
            },
        }
    }
}
