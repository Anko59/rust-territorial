use crate::modules::config::*;
use super::state::GameState;
use std::collections::HashMap;
use crate::modules::timing::ExecutionTimer;
use crate::TIMING_STATS;

impl GameState {
    pub fn update(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "game_update_full");
        
        self.process_player_updates();
        self.process_attack_movements();
        self.update_player_areas();
        self.update_grid();
    }

    fn process_player_updates(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "process_player_updates");
        
        let mut expansion_attempts = Vec::with_capacity(self.players.len());
        
        // Update resources and collect expansion attempts
        for player in &mut self.players {
            player.update_resources();
            
            if player.try_expand() {
                let investment = player.calculate_expansion_investment();
                if investment > 0 {
                    expansion_attempts.push((player.id, investment));
                }
            }
        }
        
        // Process expansion attempts in batch
        for (player_id, investment) in expansion_attempts {
            if let Some((target, is_empty)) = self.find_random_neighbor(player_id) {
                let target_id = if is_empty { usize::MAX } else { target };
                self.start_attack(player_id, target_id, investment);
            }
        }
    }

    fn process_attack_movements(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "process_attack_movements");
        
        // Collect attack updates to avoid mutable borrow conflicts
        let mut attack_updates = Vec::new();
        let mut completed_attacks = Vec::new();
        let mut investments_to_return = HashMap::new();
        
        // First pass: collect all updates
        {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "attack_movement_collection");
            for (i, attack) in self.attack_movements.iter_mut().enumerate() {
                if !attack.is_started {
                    attack.start(&self.grid);
                }
                
                let next_pixels = attack.get_next_pixels(&self.grid);
                if next_pixels.is_empty() {
                    investments_to_return.insert(attack.source, attack.investment);
                    completed_attacks.push(i);
                } else {
                    let pixel_count = next_pixels.len();
                    attack_updates.push((attack.source, next_pixels.clone()));
                    attack.investment -= pixel_count as i32;
                    
                    if attack.investment <= 0 {
                        completed_attacks.push(i);
                    }
                }
            }
        }
        
        // Second pass: apply updates
        {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "attack_movement_updates");
            for (source, pixels) in attack_updates {
                for (x, y) in pixels {
                    self.grid[y][x] = Some(source);
                }
            }

            // Process completed attacks in reverse order
            for &idx in completed_attacks.iter().rev() {
                self.attack_movements.swap_remove(idx);
            }

            // Return investments
            for (player_id, investment) in investments_to_return {
                if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
                    player.resources += investment;
                }
            }
        }
    }

    fn update_player_areas(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "update_player_areas");
        
        // Use pre-allocated vector for better performance
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
            let area = areas[player.id];
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
