use crate::modules::config::*;
use super::state::{GameState, UpdateTimers};
use super::neighbors;
use crate::modules::timing::ExecutionTimer;
use crate::modules::types::GridCell;
use crate::TIMING_STATS;
use log::debug;

impl GameState {
    pub fn update(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "game_update_full");
        
        // Initialize timers if they don't exist
        if self.update_timers.is_none() {
            self.update_timers = Some(UpdateTimers::default());
        }
        
        // Check which updates are needed
        let mut should_update_attack = false;
        let mut should_update_resources = false;
        let mut should_update_expansion = false;
        let mut should_update_areas = false;
        let mut should_update_neighbors = false;
        let mut should_update_center_of_mass = false;

        if let Some(timers) = &self.update_timers {
            should_update_attack = self.update_count - timers.last_attack_update >= ATTACK_MOVEMENT_UPDATE_INTERVAL;
            should_update_resources = self.update_count - timers.last_resource_update >= PLAYER_RESOURCE_UPDATE_INTERVAL;
            should_update_expansion = self.update_count - timers.last_expansion_update >= PLAYER_EXPANSION_UPDATE_INTERVAL;
            should_update_areas = self.update_count - timers.last_area_update >= AREA_UPDATE_INTERVAL;
            should_update_neighbors = self.update_count - timers.last_neighbor_update >= NEIGHBOR_UPDATE_INTERVAL;
            should_update_center_of_mass = self.update_count - timers.last_center_of_mass_update >= CENTER_OF_MASS_UPDATE_INTERVAL;
        }

        // Update neighbor pairs if needed
        if should_update_neighbors {
            debug!("Updating neighbor pairs...");
            self.neighbor_pairs = neighbors::find_all_possible_targets(&self.grid);
            debug!("Found {} neighbor pairs", self.neighbor_pairs.len());
        }

        // Perform updates
        if should_update_attack {
            self.process_attack_movements();
        }
        
        if should_update_resources {
            self.update_player_resources();
        }
        
        if should_update_expansion {
            debug!("Processing expansion attempts...");
            self.process_expansion_attempts();
        }
        
        if should_update_areas {
            self.update_player_areas();
        }

        if should_update_center_of_mass {
            self.update_centers_of_mass();
        }

        // Update timers
        if let Some(timers) = self.update_timers.as_mut() {
            if should_update_attack {
                timers.last_attack_update = self.update_count;
            }
            if should_update_resources {
                timers.last_resource_update = self.update_count;
            }
            if should_update_expansion {
                timers.last_expansion_update = self.update_count;
            }
            if should_update_areas {
                timers.last_area_update = self.update_count;
            }
            if should_update_neighbors {
                timers.last_neighbor_update = self.update_count;
            }
            if should_update_center_of_mass {
                timers.last_center_of_mass_update = self.update_count;
            }
        }

        // Increment update count
        self.update_count += 1;
    }

    fn update_centers_of_mass(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "update_centers_of_mass");
        
        // Create maps to store sums and counts for each player
        let mut sum_x: std::collections::HashMap<u32, f64> = std::collections::HashMap::new();
        let mut sum_y: std::collections::HashMap<u32, f64> = std::collections::HashMap::new();
        let mut counts: std::collections::HashMap<u32, i32> = std::collections::HashMap::new();

        // Calculate sums of coordinates for each player, sampling at reduction factor intervals
        for y in (0..GRID_HEIGHT).step_by(GRID_REDUCTION_FACTOR) {
            for x in (0..GRID_WIDTH).step_by(GRID_REDUCTION_FACTOR) {
                let cell = self.grid[y][x];
                if cell != GridCell::Available | GridCell::Mountain | GridCell::Water {
                    let player_id = cell.to_i32() as u32;
                    // Note: x is the horizontal coordinate (width), y is the vertical coordinate (height)
                    *sum_x.entry(player_id).or_insert(0.0) += x as f64;
                    *sum_y.entry(player_id).or_insert(0.0) += y as f64;
                    *counts.entry(player_id).or_insert(0) += 1;
                }
            }
        }

        // Update center of mass for each player
        for player in &mut self.players {
            if let (Some(&sum_x), Some(&sum_y), Some(&count)) = (
                sum_x.get(&player.id),
                sum_y.get(&player.id),
                counts.get(&player.id),
            ) {
                if count > 0 {
                    // Note: center_x corresponds to width (x), center_y corresponds to height (y)
                    player.center_x = sum_x / count as f64;
                    player.center_y = sum_y / count as f64;
                }
            }
        }
    }

    fn update_player_resources(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "update_player_resources");
        
        // Apply regular interest-based growth
        for player in &mut self.players {
            player.update_resources();
        }

        // Check if it's time for area-based bonus
        if self.update_count % BONUS_UPDATE_INTERVAL == 0 {
            // Calculate average land value for each player
            for player in &mut self.players {
                let mut total_value = 0.0;
                let mut count = 0;
                for y in 0..GRID_HEIGHT {
                    for x in 0..GRID_WIDTH {
                        if let Some(id) = self.grid[y][x] {
                            if id == player.id {
                                total_value += self.world_map.livability_map[y][x];
                                count += 1;
                            }
                        }
                    }
                }
                player.average_land_value = if count > 0 { total_value / count as f64 } else { 1.0 };
                
                // Apply area-based bonus
                let area_bonus = ((player.area as f64 * player.average_land_value) / 2.0) as i32;
                player.resources = std::cmp::min(
                    player.resources + area_bonus,
                    player.max_resources()
                );
            }
        }
    }

    fn process_expansion_attempts(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "process_expansion_attempts");
        
        // First collect player IDs and their expansion decisions
        let mut expansion_candidates = Vec::new();
        for player in &mut self.players {
            if player.try_expand() {
                debug!("Player {} wants to expand (resources: {})", player.id, player.resources);
                expansion_candidates.push(player.id);
            }
        }
        
        debug!("{} players want to expand", expansion_candidates.len());
        
        // Then process expansions using the cached neighbor pairs
        let mut expansion_attempts = Vec::new();
        for &player_id in &expansion_candidates {
            // Get player's neighbors using the neighbors module
            let mut player_neighbors = neighbors::get_neighbors(&self.neighbor_pairs, player_id);

            
            debug!("Player {} has {} possible targets (including empty space)", player_id, player_neighbors.len());
            
            // Try to get a target and investment amount
            if let Some(player) = self.players.iter().find(|p| p.id == player_id) {
                if let Some((target, investment)) = player.get_target(&player_neighbors) {
                    debug!("Player {} will attack {} with investment {}", player_id, target, investment);
                    expansion_attempts.push((player_id, target, investment));
                }
            }
        }
        
        debug!("{} attacks will be attempted", expansion_attempts.len());
        
        // Finally, process expansion attempts in batch
        for (source, target, investment) in expansion_attempts {
            self.start_attack(source, target, investment);
        }
    }

    fn process_attack_movements(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "process_attack_movements");
        
        debug!("Processing {} attack movements", self.attack_movements.len());
        
        let mut attack_updates = Vec::new();
        let mut completed_attacks = Vec::new();
        let mut investments_to_return = std::collections::HashMap::new();
        
        // First pass: collect all updates
        {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "attack_movement_collection");
            for (i, attack) in self.attack_movements.iter_mut().enumerate() {
                if !attack.is_started {
                    debug!("Starting new attack from {} to {}", attack.source, attack.target);
                    attack.start(&self.grid);
                }
                
                let next_pixels = attack.get_next_pixels(&self.grid);
                if next_pixels.is_empty() {
                    debug!("Attack from {} to {} has no next pixels, returning investment {}", 
                          attack.source, attack.target, attack.investment);
                    investments_to_return.insert(attack.source, attack.investment);
                    completed_attacks.push(i);
                } else {
                    // Find target player if it exists
                    let target_player = if attack.target != GridIndex::Available {
                        self.players.iter().find(|p| p.id == attack.target)
                    } else {
                        None
                    };

                    // Calculate costs for this attack step
                    let (source_cost, target_cost) = attack.get_next_pixels_costs(&next_pixels, target_player, &self.world_map.traversability_map);
                    
                    debug!("Attack step costs - source: {}, target: {} (pixels: {})", 
                           source_cost, target_cost, next_pixels.len());

                    // Check if attack can proceed
                    if attack.investment >= source_cost {
                        // Deduct resources from target if it exists
                        if let Some(target) = target_player {
                            if target.resources < target_cost {
                                debug!("Target {} doesn't have enough resources ({} < {})", 
                                      target.id, target.resources, target_cost);
                                completed_attacks.push(i);
                                continue;
                            }
                        }

                        // Update attack state
                        attack_updates.push((attack.source, attack.target, next_pixels.clone(), source_cost, target_cost));
                        attack.border_pixels = next_pixels;
                        attack.investment -= source_cost;
                        
                        debug!("Attack from {} to {} continues - remaining investment: {}", 
                               attack.source, attack.target, attack.investment);

                        if attack.investment <= 0 {
                            debug!("Attack from {} to {} depleted investment", attack.source, attack.target);
                            completed_attacks.push(i);
                        }
                    } else {
                        debug!("Attack from {} to {} cannot afford next step (cost: {}, remaining: {})", 
                               attack.source, attack.target, source_cost, attack.investment);
                        investments_to_return.insert(attack.source, attack.investment);
                        completed_attacks.push(i);
                    }
                }
            }
        }
        
        // Second pass: apply updates
        {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "attack_movement_updates");
            
            // Apply grid updates and deduct target resources
            for (source, target, pixels, _, target_cost) in attack_updates {
                // Update grid
                for (x, y) in pixels {
                    self.grid[y][x] = Some(source);
                }

                // Deduct resources from target if it exists and isn't empty space
                if target != GridIndex::Available {
                    if let Some(target_player) = self.players.iter_mut().find(|p| p.id == target) {
                        target_player.resources -= target_cost;
                    }
                }
            }

            // Process completed attacks in reverse order
            for &idx in completed_attacks.iter().rev() {
                self.attack_movements.swap_remove(idx);
            }

            // Return unused investments
            for (player_id, investment) in investments_to_return {
                if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
                    player.resources += investment;
                }
            }
        }
        
        debug!("{} attack movements remaining", self.attack_movements.len());
    }

    fn update_player_areas(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "update_player_areas");
        
        let mut areas = vec![0; NUM_PLAYERS];
        
        // Process grid in chunks for better cache utilization
        {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "area_calculation");
            for chunk in self.grid.chunks(64) {
                for row in chunk {
                    for cell in row {
                        if let Some(id) = cell {
                            if *id < NUM_PLAYERS {
                                areas[*id] += 1;
                            }
                        }
                    }
                }
            }
        }

        // Collect players to eliminate
        let mut to_eliminate = Vec::new();
        for player in &mut self.players {
            let area = areas[player.id - 1];
            if area < MIN_AREA_THRESHOLD {
                to_eliminate.push(player.id);
            } else {
                player.area = area;
            }
        }

        // Eliminate players
        {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "player_elimination");
            for player_id in to_eliminate {
                self.eliminate_player(player_id);
                if let Some(pos) = self.players.iter().position(|p| p.id == player_id) {
                    self.players.swap_remove(pos);
                }
            }
        }
    }
}
