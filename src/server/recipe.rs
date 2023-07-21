// For now, since most of this will be unused a while
#![allow(dead_code)]

use std::convert::TryInto;

use crate::nbt::NBTTag;
use crate::packets::packet_writer::PacketWriter;

#[derive(Debug, Clone)]
pub struct Slot {
    pub present: bool,
    pub item_id: Option<i32>,
    pub count: Option<u8>,
    pub nbt: Option<NBTTag>,
}

impl Slot {
    pub fn write(&self, writer: &mut PacketWriter) {
        writer.add_boolean(self.present);

        if let Some(item_id) = self.item_id {
            writer.add_varint(item_id);
        }

        if let Some(count) = self.count {
            writer.add_unsigned_byte(count);
        }

        if let Some(nbt) = self.nbt.clone() {
            writer.add_nbt(&nbt);
        }
    }
}

type Ingredient = Vec<Slot>;

#[derive(Debug, Clone)]
pub enum RecipeData {
    // Group, Ingredients, Result
    CraftingShapeless(String, Vec<Ingredient>, Slot),

    // Width, Height, Group, Ingredients, Result
    CraftingShaped(i32, i32, String, Vec<Ingredient>, Slot),

    // No arguments
    CraftingSpecialArmordye(),
    CraftingSpecialBookcloning(),
    CraftingSpecialMapcloning(),
    CraftingSpecialMapextending(),
    CraftingSpecialFireworkRocket(),
    CraftingSpecialFireworkStar(),
    CraftingSpecialFireworkStarFade(),
    CraftingSpecialRepairitem(),
    CraftingSpecialTippedarrow(),
    CraftingSpecialBannerduplicate(),
    CraftingSpecialBanneraddpattern(),
    CraftingSpecialShielddecoration(),
    CraftingSpecialShulkerboxcoloring(),
    CraftingSpecialSuspiciousstew(),

    // Group, Ingredient, Result, Experience, Cooking Time
    Smelting(String, Ingredient, Slot, f32, i32),
    Blasting(String, Ingredient, Slot, f32, i32),
    Smoking(String, Ingredient, Slot, f32, i32),
    CampfireCooking(String, Ingredient, Slot, f32, i32),

    // Group, Ingredient, Result
    StoneCutting(String, Ingredient, Slot),

    // Base, Addition, Result
    Smithing(Ingredient, Ingredient, Slot),
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: String,
    pub data: RecipeData,
}

impl Recipe {
    pub fn write(&self, writer: &mut PacketWriter) {
        match &self.data {
            RecipeData::CraftingShapeless(group, ingredients, result) => {
                writer.add_string(&"crafting_shapeless".to_string());

                writer.add_string(&self.id);

                writer.add_string(&group);

                writer.add_varint(ingredients.len().try_into().unwrap());
                for ingredient in ingredients {
                    writer.add_varint(ingredient.len().try_into().unwrap());
                    for slot in ingredient {
                        if !slot.present || slot.count != Some(1) || slot.item_id.is_none() || slot.nbt.is_none() {
                            eprintln!("Invalid item in recipe data: {:?}", slot);
                        }
                        slot.write(writer);
                    }
                }

                result.write(writer);
            }
            _ => todo!()
        }
    }
}
