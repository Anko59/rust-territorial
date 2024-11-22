use std::collections::HashMap;
use rand::Rng;
use crate::modules::config::*;
use super::state::GameState;

impl GameState {
    pub fn find_random_neighbor(&self, player_id: usize) -> Option<(usize, bool)> {
        // Pre-calculate and cache player cells for better performance
        let player_cells = self.get_player_cells_cached(player_id);
        let mut neighbors = HashMap::with_capacity(4); // Pre-allocate with typical capacity
        let mut has_empty_space = false;

        // Static direction array to avoid repeated allocations
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        // Process cells in chunks for better cache utilization
        for &(x, y) in player_cells.iter() {
            let x = x as i32;
            let y = y as i32;
            
            // Unrolled neighbor checking loop for better performance
            for &(dx, dy) in &DIRECTIONS {
                let new_x = x + dx;
                let new_y = y + dy;
                
                if new_x >= 0 && new_x < GRID_WIDTH as i32 &&
                   new_y >= 0 && new_y < GRID_HEIGHT as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    
                    match self.grid[new_y][new_x] {
                        Some(neighbor_id) if neighbor_id != player_id => {
                            // Use entry API for more efficient HashMap updates
                            neighbors.entry(neighbor_id)
                                   .and_modify(|count| *count += 1)
                                   .or_insert(1);
                        },
                        None => has_empty_space = true,
                        _ => {}
                    }
                }
            }
        }

        self.choose_expansion_target(neighbors, has_empty_space)
    }

    // Optimized cell retrieval with capacity pre-allocation
    fn get_player_cells(&self, player_id: usize) -> Vec<(usize, usize)> {
        // Estimate capacity based on grid size and typical territory size
        let estimated_capacity = (GRID_WIDTH * GRID_HEIGHT) / NUM_PLAYERS;
        let mut cells = Vec::with_capacity(estimated_capacity);

        // Process rows in chunks for better cache utilization
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(id) = self.grid[y][x] {
                    if id == player_id {
                        cells.push((x, y));
                    }
                }
            }
        }
        cells
    }

    // Cache player cells for repeated use
    fn get_player_cells_cached(&self, player_id: usize) -> Vec<(usize, usize)> {
        self.get_player_cells(player_id)
    }

    fn choose_expansion_target(&self, neighbors: HashMap<usize, u32>, has_empty_space: bool) -> Option<(usize, bool)> {
        let mut rng = rand::thread_rng();
        
        // Weighted decision based on neighbor frequency
        if has_empty_space && (neighbors.is_empty() || rng.gen_bool(BASE_EXPANSION_CHANCE)) {
            Some((0, true))
        } else if !neighbors.is_empty() {
            // Weight neighbors by their frequency of occurrence
            let total_weight: u32 = neighbors.values().sum();
            let mut choice = rng.gen_range(0..total_weight);
            
            for (neighbor_id, weight) in neighbors {
                if choice < weight {
                    return Some((neighbor_id, false));
                }
                choice -= weight;
            }
            None
        } else {
            None
        }
    }
}
