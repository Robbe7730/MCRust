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

impl RecipeData {
    pub fn get_tag(&self) -> String {
        match self {
            RecipeData::CraftingShapeless(_, _, _) => "crafting_shapeless".to_string(),
            RecipeData::CraftingShaped(_, _, _, _, _) => "crafting_shaped".to_string(),
            RecipeData::CraftingSpecialArmordye() => "crafting_special_armordye".to_string(),
            RecipeData::CraftingSpecialBookcloning() => "crafting_special_bookcloning".to_string(),
            RecipeData::CraftingSpecialMapcloning() => "crafting_special_mapcloning".to_string(),
            RecipeData::CraftingSpecialMapextending() => "crafting_special_mapextending".to_string(),
            RecipeData::CraftingSpecialFireworkRocket() => "crafting_special_firework_rocket".to_string(),
            RecipeData::CraftingSpecialFireworkStar() => "crafting_special_Firework_star".to_string(),
            RecipeData::CraftingSpecialFireworkStarFade() => "crafting_special_Firework_star_fade".to_string(),
            RecipeData::CraftingSpecialRepairitem() => "crafting_special_repairitem".to_string(),
            RecipeData::CraftingSpecialTippedarrow() => "crafting_special_tippedarrow".to_string(),
            RecipeData::CraftingSpecialBannerduplicate() => "crafting_special_bannerduplicate".to_string(),
            RecipeData::CraftingSpecialBanneraddpattern() => "crafting_special_banneraddpattern".to_string(),
            RecipeData::CraftingSpecialShielddecoration() => "crafting_special_shielddecoration".to_string(),
            RecipeData::CraftingSpecialShulkerboxcoloring() => "crafting_special_shulkerboxcoloring".to_string(),
            RecipeData::CraftingSpecialSuspiciousstew() => "crafting_special_suspiciousstew".to_string(),
            RecipeData::Smelting(_, _, _, _, _) => "smelting".to_string(),
            RecipeData::Blasting(_, _, _, _, _) => "blasting".to_string(),
            RecipeData::Smoking(_, _, _, _, _) => "smoking".to_string(),
            RecipeData::CampfireCooking(_, _, _, _, _) => "campfire_cooking".to_string(),
            RecipeData::StoneCutting(_, _, _) => "stone_cutting".to_string(),
            RecipeData::Smithing(_, _, _) => "smithing".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub id: String,
    pub data: RecipeData,
}

impl Recipe {
    pub fn write(&self, writer: &mut PacketWriter) {
        writer.add_string(&self.data.get_tag());
        writer.add_string(&self.id);
        match &self.data {
            RecipeData::CraftingShapeless(group, ingredients, result) => {
                writer.add_string(&group);

                writer.add_varint(ingredients.len().try_into().unwrap());
                for ingredient in ingredients {
                    Self::write_ingredient(ingredient, writer);
                }

                result.write(writer);
            }
            RecipeData::CraftingShaped(width, height, group, ingredients, result) => {
                writer.add_varint(*width);
                writer.add_varint(*height);
                writer.add_string(&group);

                for ingredient in ingredients {
                    Self::write_ingredient(ingredient, writer);
                }

                result.write(writer);
            }
            RecipeData::CraftingSpecialArmordye() => {}
            RecipeData::CraftingSpecialBookcloning() => {}
            RecipeData::CraftingSpecialMapcloning() => {}
            RecipeData::CraftingSpecialMapextending() => {}
            RecipeData::CraftingSpecialFireworkRocket() => {}
            RecipeData::CraftingSpecialFireworkStar() => {}
            RecipeData::CraftingSpecialFireworkStarFade() => {}
            RecipeData::CraftingSpecialRepairitem() => {}
            RecipeData::CraftingSpecialTippedarrow() => {}
            RecipeData::CraftingSpecialBannerduplicate() => {}
            RecipeData::CraftingSpecialBanneraddpattern() => {}
            RecipeData::CraftingSpecialShielddecoration() => {}
            RecipeData::CraftingSpecialShulkerboxcoloring() => {}
            RecipeData::CraftingSpecialSuspiciousstew() => {}
            RecipeData::Smelting(group, ingredient, result, xp, cooking_time) | 
                RecipeData::Blasting(group, ingredient, result, xp, cooking_time) | 
                RecipeData::CampfireCooking(group, ingredient, result, xp, cooking_time) | 
                RecipeData::Smoking(group, ingredient, result, xp, cooking_time) => {
                    writer.add_string(group);
                    Self::write_ingredient(ingredient, writer);
                    result.write(writer);
                    writer.add_float(*xp);
                    writer.add_varint(*cooking_time);
            }
            RecipeData::StoneCutting(group, ingredient, result) => {
                writer.add_string(group);
                Self::write_ingredient(ingredient, writer);
                result.write(writer);
            }
            RecipeData::Smithing(base, addition, result) => {
                Self::write_ingredient(base, writer);
                Self::write_ingredient(addition, writer);
                result.write(writer);
            }
        }
    }

    fn write_ingredient(ingredient: &Ingredient, writer: &mut PacketWriter) {
        writer.add_varint(ingredient.len().try_into().unwrap());
        for slot in ingredient {
            slot.write(writer);
        }
    }
}
