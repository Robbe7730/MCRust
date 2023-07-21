use std::fmt::Debug;

use super::Dimension;

use crate::util::Gamemode;

use uuid::Uuid;

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

#[derive(Clone, Copy)]
pub struct Abilities {
    pub value: u8,
}

pub enum Ability {
    Invulnerable,
    Flying,
    AllowFlying,
    CreativeMode,
}

impl Abilities {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn add_ability(self, ability: Ability) -> Self {
        let new_ability_bitmask = match ability {
            Ability::Invulnerable => 0b0001,
            Ability::Flying       => 0b0010,
            Ability::AllowFlying  => 0b0100,
            Ability::CreativeMode => 0b1000,
        };
        return Self {
            value: self.value | new_ability_bitmask
        }
    }
}

impl Debug for Abilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value & 0b0001 != 0 {
            write!(f, "Invulnerable")?;
        }

        if self.value & 0b0010 != 0 {
            if self.value & 0b0001 != 0 {
                write!(f, " | ")?;
            }
            write!(f, "Flying")?;
        }

        if self.value & 0b0100 != 0 {
            if self.value & 0b0011 != 0 {
                write!(f, " | ")?;
            }
            write!(f, "AllowFlying")?;
        }

        if self.value & 0b1000 != 0 {
            if self.value & 0b0111 != 0 {
                write!(f, " | ")?;
            }
            write!(f, "CreativeMode")?;
        }

        Ok(())
    }
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
        }
    }
}
