use crate::nbt::NBTTag;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum InfiniburnType {
    Overworld,
}

impl Into<NBTTag> for InfiniburnType {
    fn into(self) -> NBTTag {
        match self {
            InfiniburnType::Overworld => "minecraft:infiniburn_overworld".into(),
        }
    }
}

// These elements may change over time, these are the ones the 1.16.4 Notchian client asked for
#[derive(Clone, Debug)]
pub struct DimensionSettings {
    pub ambient_light: f32,
    pub infiniburn: InfiniburnType,
    pub logical_height: i32,
    pub has_raids: bool,
    pub respawn_anchor_works: bool,
    pub bed_works: bool,
    pub piglin_safe: bool,
    pub coordinate_scale: f32,
    pub natural: bool,
    pub ultrawarm: bool,
    pub has_ceiling: bool,
    pub has_skylight: bool,
}

impl Into<NBTTag> for DimensionSettings {
    fn into(self) -> NBTTag {
        let mut ret = HashMap::new();
        ret.insert("ambient_light", self.ambient_light.into());
        ret.insert("infiniburn", self.infiniburn.into());
        ret.insert("logical_height", self.logical_height.into());
        ret.insert("has_raids", self.has_raids.into());
        ret.insert("respawn_anchor_works", self.respawn_anchor_works.into());
        ret.insert("bed_works", self.bed_works.into());
        ret.insert("piglin_safe", self.piglin_safe.into());
        ret.insert("coordinate_scale", self.coordinate_scale.into());
        ret.insert("natural", self.natural.into());
        ret.insert("ultrawarm", self.ultrawarm.into());
        ret.insert("has_ceiling", self.has_ceiling.into());
        ret.insert("has_skylight", self.has_skylight.into());
        ret.into()
    }
}

#[derive(Debug, Clone)]
pub struct Dimension {
    pub name: String, // TODO: make this an identifier
    pub id: i32,
    pub settings: DimensionSettings,
}

impl Dimension {
    pub fn dummy() -> Self {
        Self {
            name: "mcrust:the_only_dimension".to_string(),
            id: 0,
            settings: DimensionSettings {
                // Based on the overworld
                ambient_light: 0.0,
                infiniburn: InfiniburnType::Overworld,
                logical_height: 256,
                has_raids: true,
                respawn_anchor_works: false,
                bed_works: true,
                piglin_safe: false,
                coordinate_scale: 1.0,
                natural: true,
                ultrawarm: false,
                has_ceiling: false,
                has_skylight: true,
            },
        }
    }
}

impl Into<NBTTag> for Dimension {
    fn into(self) -> NBTTag {
        let mut ret = HashMap::new();
        ret.insert("name", self.name.into());
        ret.insert("id", self.id.into());
        ret.insert("element", self.settings.into());
        ret.into()
    }
}
