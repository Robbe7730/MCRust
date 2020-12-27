use super::Dimension;

use crate::util::Gamemode;

use uuid::Uuid;

#[derive(Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub username: String,
    pub gamemode: Gamemode,
    pub previous_gamemode: Option<Gamemode>,
    pub dimension: Dimension,
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
