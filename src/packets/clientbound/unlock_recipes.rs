use std::convert::TryInto;

use super::Clientbound;

use crate::{packets::packet_writer::PacketWriter, player::Player};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnlockRecipesAction {
    Init,
    Add,
    Remove
}

impl Into<u8> for UnlockRecipesAction {
    fn into(self) -> u8 {
        match self {
            UnlockRecipesAction::Init => 0,
            UnlockRecipesAction::Add => 1,
            UnlockRecipesAction::Remove => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnlockRecipesPacket {
    pub action: UnlockRecipesAction,
    pub crafting_table_open: bool,
    pub crafting_table_filter: bool,
    pub furnace_open: bool,
    pub furnace_filter: bool,
    pub blast_furnace_open: bool,
    pub blast_furnace_filter: bool,
    pub smoker_open: bool,
    pub smoker_filter: bool,
    pub recipes_list1: Vec<String>,
    pub recipes_list2: Option<Vec<String>>,
}

impl Clientbound for UnlockRecipesPacket {
    fn writer(&self) -> PacketWriter {
        let mut writer = PacketWriter::new(0x35);

        writer.add_unsigned_byte(self.action.into());
        writer.add_boolean(self.crafting_table_open);
        writer.add_boolean(self.crafting_table_filter);
        writer.add_boolean(self.furnace_open);
        writer.add_boolean(self.furnace_filter);
        writer.add_boolean(self.blast_furnace_open);
        writer.add_boolean(self.blast_furnace_filter);
        writer.add_boolean(self.smoker_open);
        writer.add_boolean(self.smoker_filter);
        writer.add_varint(self.recipes_list1.len().try_into().unwrap());
        for recipe_id in self.recipes_list1.iter() {
            writer.add_string(&recipe_id);
        }

        if self.action == UnlockRecipesAction::Init {
            if let Some(list2) = self.recipes_list2.as_ref() {
                writer.add_varint(list2.len().try_into().unwrap());
                for recipe_id in list2.iter() {
                    writer.add_string(&recipe_id);
                }
            } else {
                eprintln!("Missing list 2 for init unlocked recipes!");
            }
        } else if self.recipes_list2.is_some() {
            eprintln!("Recipe list 2 should only be used in action Init, not {:?}", self.action);
        }

        writer
    }
}

impl UnlockRecipesPacket {
    pub fn init_from_player(player: &Player) -> Self {
        Self {
            action: UnlockRecipesAction::Init,
            crafting_table_open: player.recipe_book_state.crafting_table_open,
            crafting_table_filter: player.recipe_book_state.crafting_table_filter,
            furnace_open: player.recipe_book_state.furnace_open,
            furnace_filter: player.recipe_book_state.furnace_filter,
            blast_furnace_open: player.recipe_book_state.blast_furnace_open,
            blast_furnace_filter: player.recipe_book_state.blast_furnace_filter,
            smoker_open: player.recipe_book_state.smoker_open,
            smoker_filter: player.recipe_book_state.smoker_filter,
            recipes_list1: player.unlocked_recipes.clone(),
            recipes_list2: Some(player.unlocked_recipes.clone()),
        }
    }
}
