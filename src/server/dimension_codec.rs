use super::Biome;
use super::Dimension;

use crate::nbt::NBTTag;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DimensionCodec {
    pub dimensions: HashMap<String, Dimension>,
    pub biomes: HashMap<String, Biome>,
}

impl DimensionCodec {
    pub fn new() -> Self {
        Self {
            dimensions: HashMap::new(),
            biomes: HashMap::new(),
        }
    }

    pub fn add_dimension(&mut self, dim: Dimension) {
        self.dimensions.insert(dim.name.clone(), dim.clone());
    }

    pub fn add_biome(&mut self, biome: Biome) {
        self.biomes.insert(biome.name.clone(), biome.clone());
    }
}

impl Into<NBTTag> for DimensionCodec {
    fn into(self) -> NBTTag {
        let mut dimension_types = HashMap::new();
        dimension_types.insert("type", "minecraft:dimension_type".into());
        dimension_types.insert(
            "value",
            self.dimensions
                .values()
                .map(|x| x.clone().into())
                .collect::<Vec<NBTTag>>()
                .into(),
        );

        let mut biome_types = HashMap::new();
        biome_types.insert("type", "minecraft:worldgen/biome".into());
        biome_types.insert(
            "value",
            self.biomes
                .values()
                .map(|x| x.clone().into())
                .collect::<Vec<NBTTag>>()
                .into(),
        );

        let mut ret = HashMap::new();
        ret.insert("minecraft:dimension_type", dimension_types.into());
        ret.insert("minecraft:worldgen/biome", biome_types.into());
        ret.into()
    }
}
