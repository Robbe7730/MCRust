use crate::nbt::NBTTag;

use std::collections::HashMap;

#[derive(Clone)]
#[allow(dead_code)]
pub enum BiomeCategory {
    None,
    Ocean,
    Plains,
    Desert,
    ExtremeHills,
    Forest,
    Taiga,
    Swamp,
    River,
    Nether,
    TheEnd,
    Icy,
    Mushroom,
    Beach,
    Jungle,
    Savanna,
    Mesa,
}

impl Into<NBTTag> for BiomeCategory {
    fn into(self) -> NBTTag {
        match self {
            BiomeCategory::None => "none".into(),
            BiomeCategory::Ocean => "ocean".into(),
            BiomeCategory::Plains => "plains".into(),
            BiomeCategory::Desert => "desert".into(),
            BiomeCategory::ExtremeHills => "extreme_hills".into(),
            BiomeCategory::Forest => "forest".into(),
            BiomeCategory::Taiga => "taiga".into(),
            BiomeCategory::Swamp => "swamp".into(),
            BiomeCategory::River => "river".into(),
            BiomeCategory::Nether => "nether".into(),
            BiomeCategory::TheEnd => "the_end".into(),
            BiomeCategory::Icy => "icy".into(),
            BiomeCategory::Mushroom => "mushroom".into(),
            BiomeCategory::Beach => "beach".into(),
            BiomeCategory::Jungle => "jungle".into(),
            BiomeCategory::Savanna => "savanna".into(),
            BiomeCategory::Mesa => "mesa".into(),
        }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum PrecipitationType {
    None,
    Rain,
    Snow,
}

impl Into<NBTTag> for PrecipitationType {
    fn into(self) -> NBTTag {
        match self {
            PrecipitationType::None => "none".into(),
            PrecipitationType::Rain => "rain".into(),
            PrecipitationType::Snow => "snow".into(),
        }
    }
}

#[derive(Clone)]
pub struct BiomeEffects {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub water_color: i32,
    pub fog_color: i32,
}

impl Into<NBTTag> for BiomeEffects {
    fn into(self) -> NBTTag {
        let mut ret = HashMap::new();
        ret.insert("sky_color", self.sky_color.into());
        ret.insert("water_fog_color", self.water_fog_color.into());
        ret.insert("water_color", self.water_color.into());
        ret.insert("fog_color", self.fog_color.into());
        ret.into()
    }
}

#[derive(Clone)]
pub struct BiomeSettings {
    pub scale: f32,
    pub depth: f32,
    pub category: BiomeCategory,
    pub precipitation: PrecipitationType,
    pub downfall: f32,
    pub temperature: f32,
    pub effects: BiomeEffects,
}

impl Into<NBTTag> for BiomeSettings {
    fn into(self) -> NBTTag {
        let mut ret = HashMap::new();
        ret.insert("scale", self.scale.into());
        ret.insert("depth", self.depth.into());
        ret.insert("category", self.category.into());
        ret.insert("precipitation", self.precipitation.into());
        ret.insert("downfall", self.downfall.into());
        ret.insert("temperature", self.temperature.into());
        ret.insert("effects", self.effects.into());
        ret.into()
    }
}

#[derive(Clone)]
pub struct Biome {
    pub id: i32,
    pub name: String,
    pub settings: BiomeSettings,
}

impl Biome {
    pub fn dummy() -> Self {
        // For some reason, a "minecraft:plains" biome is required for the Notchian server to work
        Self {
            id: 0,
            name: "minecraft:plains".to_string(),
            settings: BiomeSettings {
                scale: 0.05,
                depth: 0.125,
                category: BiomeCategory::Plains,
                precipitation: PrecipitationType::Rain,
                downfall: 0.4,
                temperature: 0.8,
                effects: BiomeEffects {
                    sky_color: 0x78a7ff,
                    water_fog_color: 0x050533,
                    water_color: 0x3f76e4,
                    fog_color: 0xc0d8ff,
                },
            },
        }
    }
}

impl Into<NBTTag> for Biome {
    fn into(self) -> NBTTag {
        let mut ret = HashMap::new();
        ret.insert("id", self.id.into());
        ret.insert("name", self.name.into());
        ret.insert("element", self.settings.into());
        ret.into()
    }
}
