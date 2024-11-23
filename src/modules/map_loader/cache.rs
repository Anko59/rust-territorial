use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use log::{info, debug};
use serde_json;
use sha2::{Sha256, Digest};
use crate::modules::types::WorldMap;

pub struct CacheManager {
    cache_dir: PathBuf,
    seed: i32,
}

impl CacheManager {
    pub fn new(seed: i32) -> Result<Self, Box<dyn Error>> {
        let cache_dir = Path::new("map_cache").to_path_buf();
        std::fs::create_dir_all(&cache_dir)?;
        Ok(CacheManager { cache_dir, seed })
    }

    fn generate_cache_key(&self, width: usize, height: usize) -> String {
        let key = format!("{}_{}_{}_{}", width, height, self.seed, crate::modules::config::MAP_VERSION);
        format!("{:x}", Sha256::digest(key.as_bytes()))
    }

    fn get_cache_path(&self, cache_key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", cache_key))
    }

    pub fn load_cached_data(&self, width: usize, height: usize) -> Option<WorldMap> {
        let cache_key = self.generate_cache_key(width, height);
        let cache_path = self.get_cache_path(&cache_key);

        if cache_path.exists() {
            match File::open(&cache_path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    match serde_json::from_reader(reader) {
                        Ok(world_map) => {
                            info!("Loaded map data from cache");
                            return Some(world_map);
                        }
                        Err(e) => {
                            debug!("Failed to parse cached data: {}", e);
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to open cache file: {}", e);
                }
            }
        }
        None
    }

    pub fn save_cached_data(&self, world_map: &WorldMap, width: usize, height: usize) -> Result<(), Box<dyn Error>> {
        let cache_key = self.generate_cache_key(width, height);
        let cache_path = self.get_cache_path(&cache_key);
        let file = File::create(cache_path)?;
        serde_json::to_writer(file, world_map)?;
        info!("Saved map data to cache");
        Ok(())
    }
}
