use rand::Rng;
use crate::modules::config::*;
use crate::modules::types::{Player, GridCell};
use crate::modules::map_loader::MapLoader;
use super::state::GameState;
use super::neighbors;
use log::{debug, error};

pub fn initialize_game() -> GameState {
    let mut state = GameState::new();
    let mut rng = rand::thread_rng();
    
    // Initialize world map using MapLoader
    let map_loader = match MapLoader::new(None) {
        Ok(loader) => loader,
        Err(e) => {
            error!("Failed to create MapLoader: {}", e);
            panic!("Could not initialize game due to MapLoader error");
        }
    };

    state.world_map = match map_loader.load_world_map() {
        Ok(map) => map,
        Err(e) => {
            error!("Failed to load world map: {}", e);
            panic!("Could not initialize game due to world map loading error");
        }
    };
    
    // Initialize grid with water and mountains
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if state.world_map.mountain_mask[y][x] {
                state.grid[y][x] = GridCell::Mountain; // Special value for mountains
            } else if state.world_map.water_mask[y][x] {
                state.grid[y][x] = GridCell::Water; // Special value for water
            }
        }
    }
    
    // Initialize players
    for id in 0..NUM_PLAYERS {
        let mut attempts = 0;
        let max_attempts = 1000;
        
        while attempts < max_attempts {
            let x = rng.gen_range(0..GRID_WIDTH);
            let y = rng.gen_range(0..GRID_HEIGHT);
            
            if state.is_position_available(x as i32, y as i32, 5) {
                // Create player
                let mut player = Player::new(id + 3, x, y);
                
                // Initialize territory and calculate average land value
                let mut total_value = 0.0;
                let mut count = 0;
                let radius = 3;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;
                        if new_x >= 0 && new_x < GRID_WIDTH as i32 &&
                           new_y >= 0 && new_y < GRID_HEIGHT as i32 {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;
                            state.grid[new_y][new_x] = Some(id + 3);
                            total_value += state.world_map.livability_map[new_y][new_x];
                            count += 1;
                        }
                    }
                }
                player.average_land_value = if count > 0 { total_value / count as f64 } else { 1.0 };
                state.players.push(player);
                break;
            }
            debug!("[Game] Could not find valid position for player {}", id);
            attempts += 1;
        }

        if attempts >= max_attempts {
            panic!("Could not find valid position for player {}", id);
        }
    }
    
    // Initialize neighbor pairs
    state.neighbor_pairs = neighbors::find_all_possible_targets(&state.grid);
    
    state
}
