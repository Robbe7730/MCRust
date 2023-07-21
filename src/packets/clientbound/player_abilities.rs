use crate::player::{Abilities, Player};
use crate::packets::packet_writer::PacketWriter;

use super::Clientbound;

#[derive(Debug, Clone)]
pub struct PlayerAbilitiesPacket {
    pub abilities: Abilities,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

impl Clientbound for PlayerAbilitiesPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x30);

        writer.add_unsigned_byte(self.abilities.value);
        writer.add_float(self.flying_speed);
        writer.add_float(self.fov_modifier);

        writer
    }
}

impl PlayerAbilitiesPacket {
    pub fn from_player(player: &Player) -> Self {
        return Self {
            abilities: player.abilities,
            flying_speed: player.flying_speed,
            fov_modifier: player.fov_modifier
        }
    }
}
