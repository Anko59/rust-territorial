use serde::{Serialize, Deserialize};
use log::{info, debug};
use std::collections::HashMap;
use crate::modules::config::*;
use super::biome_type::BiomeType;
use super::biome::Biome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum GridCell {
    Water,
    Mountain,
    Available,
    PlayerId(u32),
}

impl GridCell {
    pub fn from_i32(value: i32) -> Self {
        match value {
            -1 => GridCell::Water,
            -2 => GridCell::Mountain,
            0 => GridCell::Available,
            id if id > 0 => GridCell::PlayerId(id as u32),
            _ => panic!("Invalid grid cell value"),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            GridCell::Water => -1,
            GridCell::Mountain => -2,
            GridCell::Available => 0,
            GridCell::PlayerId(id) => *id as i32,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WorldMap {
    pub elevation_map: Vec<Vec<f64>>,
    pub rainfall_map: Vec<Vec<f64>>,
    pub lon_grid: Vec<Vec<f64>>,
    pub lat_grid: Vec<Vec<f64>>,
    pub temperature_map: Vec<Vec<f64>>,
    pub biome_map: Vec<Vec<BiomeType>>,
    pub traversability_map: Vec<Vec<f64>>,
    pub livability_map: Vec<Vec<f64>>,
    pub color_map: Vec<Vec<[u8; 4]>>,
    pub water_mask: Vec<Vec<bool>>,
    pub mountain_mask: Vec<Vec<bool>>,
    pub accessibility_mask: Vec<Vec<bool>>,
}

impl WorldMap {
    pub fn new(width: usize, height: usize) -> Self {
        debug!("Creating new WorldMap with dimensions {}x{}", width, height);
        let empty_map = vec![vec![0.0; width]; height];
        let empty_bool_map = vec![vec![false; width]; height];
        WorldMap {
            elevation_map: empty_map.clone(),
            rainfall_map: empty_map.clone(),
            lon_grid: empty_map.clone(),
            lat_grid: empty_map.clone(),
            temperature_map: empty_map,
            biome_map: vec![vec![BiomeType::Ocean; width]; height],
            traversability_map: vec![vec![0.0; width]; height],
            livability_map: vec![vec![0.0; width]; height],
            color_map: vec![vec![[0, 0, 0, 255]; width]; height],
            water_mask: empty_bool_map.clone(),
            mountain_mask: empty_bool_map.clone(),
            accessibility_mask: empty_bool_map,
        }
    }

    pub fn update_derived_maps(&mut self) {
        info!("Updating derived maps");
        let height = self.elevation_map.len();
        let width = self.elevation_map[0].len();
        debug!("Processing {}x{} map", width, height);

        let mut biome_counts = HashMap::new();

        for y in 0..height {
            for x in 0..width {

                let biome = self.get_biome(x, y);
                let elevation = self.elevation_map[y][x];
                *biome_counts.entry(biome.biome_type).or_insert(0) += 1;

                self.biome_map[y][x] = biome.biome_type;
                self.traversability_map[y][x] = biome.traversability;
                self.livability_map[y][x] = biome.livability;
                self.color_map[y][x] = biome.color;

                // Apply elevation shading to color
                if elevation > 0.0 {
                    let shade = (elevation / 5000.0).min(1.0).max(0.0);
                    for i in 0..3 {
                        self.color_map[y][x][i] = (
                            self.color_map[y][x][i] as f64 * (1.0 - shade) +
                            255.0 * shade
                        ) as u8;
                    }
                }

                // Update masks
                self.water_mask[y][x] = elevation <= 0.0;
                self.mountain_mask[y][x] = elevation >= MOUNTAIN_ELEVATION;
                self.accessibility_mask[y][x] = elevation > 0.0 && elevation < ACCESSIBILITY_ELEVATION;
            }
        }

        // Log biome distribution
        info!("Biome distribution:");
        for (biome_type, count) in biome_counts {
            let percentage = (count as f64 / (width * height) as f64) * 100.0;
            info!("  {:?}: {:.1}% ({} tiles)", biome_type, percentage, count);
        }
    }

    pub fn get_biome(&self, x: usize, y: usize) -> &'static Biome {
        let elevation = self.elevation_map[y][x];
        let rainfall = if self.rainfall_map[y][x].is_nan() { 0.0 } else { self.rainfall_map[y][x] };
        let temperature = self.temperature_map[y][x];

        Biome::get_biome(elevation, rainfall, temperature)
    }
}
