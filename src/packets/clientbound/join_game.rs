use super::super::packet_writer::PacketWriter;
use super::super::Clientbound;

use crate::nbt::NamedNBTTag;
use crate::server::Dimension;
use crate::server::DimensionCodec;
use crate::util::Gamemode;

use std::convert::TryInto;

#[derive(Debug)]
pub struct JoinGamePacket {
    pub entity_id: u32,
    pub is_hardcore: bool,
    pub gamemode: Gamemode,
    pub previous_gamemode: Option<Gamemode>,
    pub world_names: Vec<String>,
    pub dimension_codec: DimensionCodec,
    pub dimension: Dimension,
    pub world_name: String,
    pub hashed_seed: u64,
    pub max_players: i32,
    pub view_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

impl Clientbound for JoinGamePacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x24);
        writer.add_unsigned_int(self.entity_id);
        writer.add_unsigned_byte(self.is_hardcore as u8);
        writer.add_unsigned_byte(self.gamemode.to_byte());
        writer.add_signed_byte(
            self.previous_gamemode
                .as_ref()
                .map_or(-1, |gm| gm.to_byte() as i8),
        );
        writer.add_varint(self.world_names.len().try_into().expect("Too many worlds"));
        for world_name in &self.world_names {
            writer.add_string(&world_name);
        }
        writer.add_nbt(&NamedNBTTag::new("", self.dimension_codec.clone()));
        writer.add_nbt(&NamedNBTTag::new("", self.dimension.settings.clone()));
        writer.add_string(&self.world_name);
        writer.add_unsigned_long(self.hashed_seed);
        writer.add_varint(self.max_players);
        writer.add_varint(self.view_distance);
        writer.add_unsigned_byte(self.reduced_debug_info as u8);
        writer.add_unsigned_byte(self.enable_respawn_screen as u8);
        writer.add_unsigned_byte(self.is_debug as u8);
        writer.add_unsigned_byte(self.is_flat as u8);
        writer
    }
}
