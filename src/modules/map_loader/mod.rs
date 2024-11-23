mod cache;
mod pickle;
mod transform;
mod temperature;

use std::error::Error;
use log::{info, debug};
use crate::modules::config::*;
use crate::modules::types::WorldMap;

use self::cache::CacheManager;
use self::pickle::PickleLoader;
use self::transform::MapTransformer;
use self::temperature::TemperatureGenerator;

pub struct MapLoader {
    cache_manager: CacheManager,
}

impl MapLoader {
    pub fn new(seed: Option<i32>) -> Result<Self, Box<dyn Error>> {
        debug!("Initializing MapLoader");
        let seed = seed.unwrap_or(42);
        let cache_manager = CacheManager::new(seed)?;
        Ok(MapLoader { cache_manager })
    }

    pub fn load_world_map(&self) -> Result<WorldMap, Box<dyn Error>> {
        info!("Starting world map loading process");

        // Try loading from cache first
        if let Some(cached_map) = self.cache_manager.load_cached_data(MAP_WIDTH, MAP_HEIGHT) {
            return Ok(cached_map);
        }
        
        // Load raw map data
        debug!("Loading elevation map");
        let elevation_map = PickleLoader::load_pickle_file(ELEVATION_MAP_PATH)?;
        debug!("Loading rainfall map");
        let rainfall_map = PickleLoader::load_pickle_file(RAINFALL_MAP_PATH)?;
        debug!("Loading latitude grid");
        let lat_grid = PickleLoader::load_pickle_file(LAT_MAP_PATH)?;
        debug!("Loading longitude grid");
        let lon_grid = PickleLoader::load_pickle_file(LON_MAP_PATH)?;
        
        // Process maps
        debug!("Processing maps with Gall-Peters projection");
        let elevation_map = MapTransformer::apply_gall_peters_projection(&elevation_map);
        let rainfall_map = MapTransformer::apply_gall_peters_projection(&rainfall_map);
        let lat_grid = MapTransformer::apply_gall_peters_projection(&lat_grid);
        let lon_grid = MapTransformer::apply_gall_peters_projection(&lon_grid);
        
        // Resize maps to desired dimensions
        debug!("Resizing maps to {}x{}", MAP_WIDTH, MAP_HEIGHT);
        let elevation_map = MapTransformer::resize_map(&elevation_map, MAP_WIDTH, MAP_HEIGHT);
        let rainfall_map = MapTransformer::resize_map(&rainfall_map, MAP_WIDTH, MAP_HEIGHT);
        let lat_grid = MapTransformer::resize_map(&lat_grid, MAP_WIDTH, MAP_HEIGHT);
        let lon_grid = MapTransformer::resize_map(&lon_grid, MAP_WIDTH, MAP_HEIGHT);
        
        // Generate temperature map
        debug!("Generating temperature map");
        let temperature_map = TemperatureGenerator::generate_temperature_map(&lat_grid, &elevation_map);
        
        // Create WorldMap
        debug!("Creating WorldMap instance");
        let mut world_map = WorldMap::new(MAP_WIDTH, MAP_HEIGHT);
        
        // Convert ndarray to Vec<Vec>
        debug!("Converting arrays to Vec<Vec>");
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                world_map.elevation_map[y][x] = elevation_map[[y, x]];
                world_map.rainfall_map[y][x] = rainfall_map[[y, x]];
                world_map.temperature_map[y][x] = temperature_map[[y, x]];
                world_map.lat_grid[y][x] = lat_grid[[y, x]];
                world_map.lon_grid[y][x] = lon_grid[[y, x]];
            }
        }
        
        // Update derived maps (biomes, traversability, livability, colors)
        debug!("Updating derived maps");
        world_map.update_derived_maps();

        // Save to cache
        if let Err(e) = self.cache_manager.save_cached_data(&world_map, MAP_WIDTH, MAP_HEIGHT) {
            debug!("Failed to save cache: {}", e);
        }
        
        info!("World map loaded successfully");
        Ok(world_map)
    }
}
