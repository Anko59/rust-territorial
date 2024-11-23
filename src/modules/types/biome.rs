use std::f64;
use once_cell::sync::Lazy;
use log::debug;
use super::biome_type::BiomeType;

#[derive(Debug, Clone)]
pub struct BiomeThresholds {
    pub min_temp: f64,
    pub max_temp: f64,
    pub min_rainfall: f64,
    pub max_rainfall: f64,
    pub min_elevation: f64,
    pub max_elevation: f64,
}

#[derive(Debug, Clone)]
pub struct Biome {
    pub biome_type: BiomeType,
    pub color: [u8; 4],
    pub thresholds: BiomeThresholds,
    pub livability: f64,
    pub traversability: f64,
}

static BIOMES: Lazy<Vec<Biome>> = Lazy::new(|| {
    debug!("Initializing biome definitions");
    vec![
        Biome {
            biome_type: BiomeType::Ocean,
            color: [0, 0, 128, 255],
            thresholds: BiomeThresholds {
                min_temp: f64::NEG_INFINITY,
                max_temp: f64::INFINITY,
                min_rainfall: f64::NEG_INFINITY,
                max_rainfall: f64::INFINITY,
                min_elevation: f64::NEG_INFINITY,
                max_elevation: 0.0,
            },
            livability: 0.0,
            traversability: 0.2,
        },
        Biome {
            biome_type: BiomeType::Ice,
            color: [255, 255, 255, 255],
            thresholds: BiomeThresholds {
                min_temp: f64::NEG_INFINITY,
                max_temp: -10.0,
                min_rainfall: f64::NEG_INFINITY,
                max_rainfall: f64::INFINITY,
                min_elevation: 0.0,
                max_elevation: f64::INFINITY,
            },
            livability: 0.1,
            traversability: 0.3,
        },
        Biome {
            biome_type: BiomeType::Tundra,
            color: [224, 224, 224, 255],
            thresholds: BiomeThresholds {
                min_temp: -10.0,
                max_temp: 0.0,
                min_rainfall: 0.0,
                max_rainfall: f64::INFINITY,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.2,
            traversability: 0.6,
        },
        Biome {
            biome_type: BiomeType::ColdDesert,
            color: [200, 200, 170, 255],
            thresholds: BiomeThresholds {
                min_temp: -10.0,
                max_temp: 20.0,
                min_rainfall: 0.0,
                max_rainfall: 1.0,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.3,
            traversability: 0.7,
        },
        Biome {
            biome_type: BiomeType::Taiga,
            color: [95, 115, 62, 255],
            thresholds: BiomeThresholds {
                min_temp: 0.0,
                max_temp: 5.0,
                min_rainfall: 1.0,
                max_rainfall: f64::INFINITY,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.4,
            traversability: 0.5,
        },
        Biome {
            biome_type: BiomeType::TemperateGrassland,
            color: [167, 197, 107, 255],
            thresholds: BiomeThresholds {
                min_temp: 5.0,
                max_temp: 20.0,
                min_rainfall: 1.0,
                max_rainfall: 2.0,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.8,
            traversability: 0.9,
        },
        Biome {
            biome_type: BiomeType::TemperateForest,
            color: [76, 112, 43, 255],
            thresholds: BiomeThresholds {
                min_temp: 5.0,
                max_temp: 20.0,
                min_rainfall: 2.0,
                max_rainfall: 4.0,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.9,
            traversability: 0.7,
        },
        Biome {
            biome_type: BiomeType::TemperateRainforest,
            color: [68, 100, 18, 255],
            thresholds: BiomeThresholds {
                min_temp: 5.0,
                max_temp: 20.0,
                min_rainfall: 4.0,
                max_rainfall: f64::INFINITY,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.7,
            traversability: 0.6,
        },
        Biome {
            biome_type: BiomeType::TropicalSavanna,
            color: [177, 209, 110, 255],
            thresholds: BiomeThresholds {
                min_temp: 20.0,
                max_temp: f64::INFINITY,
                min_rainfall: 1.0,
                max_rainfall: 4.0,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.6,
            traversability: 0.8,
        },
        Biome {
            biome_type: BiomeType::TropicalForest,
            color: [66, 123, 25, 255],
            thresholds: BiomeThresholds {
                min_temp: 20.0,
                max_temp: f64::INFINITY,
                min_rainfall: 4.0,
                max_rainfall: 6.0,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.5,
            traversability: 0.5,
        },
        Biome {
            biome_type: BiomeType::TropicalRainforest,
            color: [0, 100, 0, 255],
            thresholds: BiomeThresholds {
                min_temp: 20.0,
                max_temp: f64::INFINITY,
                min_rainfall: 6.0,
                max_rainfall: f64::INFINITY,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.4,
            traversability: 0.3,
        },
        Biome {
            biome_type: BiomeType::HotDesert,
            color: [238, 218, 130, 255],
            thresholds: BiomeThresholds {
                min_temp: 20.0,
                max_temp: f64::INFINITY,
                min_rainfall: 0.0,
                max_rainfall: 1.0,
                min_elevation: 0.0,
                max_elevation: 3000.0,
            },
            livability: 0.2,
            traversability: 0.8,
        },
        Biome {
            biome_type: BiomeType::Mountain,
            color: [128, 128, 128, 255],
            thresholds: BiomeThresholds {
                min_temp: f64::NEG_INFINITY,
                max_temp: f64::INFINITY,
                min_rainfall: f64::NEG_INFINITY,
                max_rainfall: f64::INFINITY,
                min_elevation: 3000.0,
                max_elevation: f64::INFINITY,
            },
            livability: 0.3,
            traversability: 0.2,
        },
    ]
});

impl Biome {
    pub fn get_biome(elevation: f64, rainfall: f64, temperature: f64) -> &'static Biome {

        if elevation <= 0.0 {
            return BIOMES.iter()
                .find(|b| b.biome_type == BiomeType::Ocean)
                .unwrap();
        }

        let matching_biomes: Vec<_> = BIOMES.iter()
            .filter(|b| {
                b.thresholds.min_elevation <= elevation && elevation < b.thresholds.max_elevation &&
                b.thresholds.min_rainfall <= rainfall && rainfall < b.thresholds.max_rainfall &&
                b.thresholds.min_temp <= temperature && temperature < b.thresholds.max_temp
            })
            .collect();

        if !matching_biomes.is_empty() {
            matching_biomes[0]
        } else {
            // Fallback logic
            let biome = if elevation >= 3000.0 {
                BIOMES.iter().find(|b| b.biome_type == BiomeType::Mountain).unwrap()
            } else if temperature < -10.0 {
                BIOMES.iter().find(|b| b.biome_type == BiomeType::Ice).unwrap()
            } else if temperature < 20.0 {
                BIOMES.iter().find(|b| b.biome_type == BiomeType::ColdDesert).unwrap()
            } else {
                BIOMES.iter().find(|b| b.biome_type == BiomeType::HotDesert).unwrap()
            };
            biome
        }
    }
}
