use super::Player;

#[derive(Clone)]
pub enum Entity {
    PlayerEntity(Player),
}

impl Entity {
    pub fn as_player(&self) -> Option<&Player> {
        match self {
            Entity::PlayerEntity(p) => Some(p),
        }
    }
}
