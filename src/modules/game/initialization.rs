use rand::Rng;
use crate::modules::config::*;
use crate::modules::types::Player;
use super::state::GameState;

impl GameState {
    pub fn initialize_players(&mut self) {
        println!("Initializing {} players on {}x{} grid...", NUM_PLAYERS, GRID_WIDTH, GRID_HEIGHT);
        let mut rng = rand::thread_rng();
        
        // Calculate optimal placement parameters for larger grid
        let placement_radius = 3_usize;
        let min_distance = (GRID_WIDTH * GRID_HEIGHT / NUM_PLAYERS) as f64;
        let spacing = (min_distance.sqrt() as usize).max(placement_radius * 2);
        
        println!("Grid spacing: {}, placement radius: {}", spacing, placement_radius);
        
        // Pre-calculate grid sections for better distribution
        let sections_x = GRID_WIDTH / spacing;
        let sections_y = GRID_HEIGHT / spacing;
        let mut available_sections: Vec<(usize, usize)> = (0..sections_x)
            .flat_map(|x| (0..sections_y).map(move |y| (x, y)))
            .collect();
        
        println!("Created {}x{} grid sections ({} total)", sections_x, sections_y, available_sections.len());
        
        for id in 0..NUM_PLAYERS {
            if available_sections.is_empty() {
                println!("Warning: No more sections available for player {}", id);
                continue;
            }
            
            // Select a random available section
            let section_idx = rng.gen_range(0..available_sections.len());
            let (section_x, section_y) = available_sections.swap_remove(section_idx);
            
            // Calculate position within section
            let base_x = section_x * spacing;
            let base_y = section_y * spacing;
            let offset_range = spacing - (placement_radius * 2);
            
            // Try to find an available position in the section
            let mut found_position = false;
            let mut attempts = 0;
            let max_attempts = 10;
            
            while !found_position && attempts < max_attempts {
                let x = (base_x + placement_radius + rng.gen_range(0..offset_range))
                    .min(GRID_WIDTH - placement_radius - 1);
                let y = (base_y + placement_radius + rng.gen_range(0..offset_range))
                    .min(GRID_HEIGHT - placement_radius - 1);
                
                if self.is_position_available(x as i32, y as i32, placement_radius as i32) {
                    println!("Placing player {} in section ({}, {}) at position ({}, {})", 
                        id, section_x, section_y, x, y);
                    
                    let player = Player::new(id, x, y);
                    self.players.push(player);
                    
                    // Create initial territory efficiently
                    self.create_initial_territory(x, y, id);
                    found_position = true;
                }
                
                attempts += 1;
            }
            
            if !found_position {
                println!("Warning: Could not find valid position for player {} in section ({}, {})", 
                    id, section_x, section_y);
            }
        }
        
        println!("Successfully initialized {} players", self.players.len());
    }

    fn create_initial_territory(&mut self, x: usize, y: usize, id: usize) {
        // Pre-calculate bounds for better performance
        let min_x = x.saturating_sub(1);
        let max_x = (x + 1).min(GRID_WIDTH - 1);
        let min_y = y.saturating_sub(1);
        let max_y = (y + 1).min(GRID_HEIGHT - 1);
        
        // Batch update territory
        for y in min_y..=max_y {
            let row = &mut self.grid[y];
            for x in min_x..=max_x {
                row[x] = Some(id);
            }
        }
        
        println!("Created initial territory for player {} at ({}, {}) with bounds: x={}..{}, y={}..{}", 
            id, x, y, min_x, max_x, min_y, max_y);
    }
}
