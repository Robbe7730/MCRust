use std::fmt::Debug;

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
