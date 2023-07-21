use crate::player::Player;

use crate::error_type::ErrorType;

#[derive(Clone)]
pub enum Entity {
    PlayerEntity(Player),
}

impl Entity {
    pub fn as_player(&self) -> Result<&Player, ErrorType> {
        match self {
            Entity::PlayerEntity(p) => Ok(p),
        }
    }

    pub fn as_player_mut(&mut self) -> Result<&mut Player, ErrorType> {
        match self {
            Entity::PlayerEntity(p) => Ok(p),
        }
    }
}
