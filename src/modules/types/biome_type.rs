use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum BiomeType {
    Ocean = 0,
    Tundra = 1,
    Taiga = 2,
    TemperateGrassland = 3,
    TemperateForest = 4,
    TemperateRainforest = 5,
    TropicalSavanna = 6,
    TropicalForest = 7,
    TropicalRainforest = 8,
    Desert = 9,
    Mountain = 10,
    Ice = 11,
    ColdDesert = 12,
    HotDesert = 13,
}
