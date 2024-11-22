use std::collections::HashSet;
use serde::Serialize;
use crate::modules::config::*;
use crate::modules::types::{Grid, Players, AttackMovement};
use crate::modules::timing::ExecutionTimer;
use crate::TIMING_STATS;

#[derive(Clone, Serialize)]
pub struct GameState {
    pub grid: Grid,
    pub players: Players,
    pub(crate) attack_movements: Vec<AttackMovement>,
}

impl GameState {
    pub fn new() -> Self {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "game_state_creation");
        let grid = vec![vec![None; GRID_WIDTH]; GRID_HEIGHT];
        let players = Vec::new();
        let attack_movements = Vec::new();
        GameState { grid, players, attack_movements }
    }

    pub fn is_position_available(&self, x: i32, y: i32, radius: i32) -> bool {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "check_position_availability");
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0 && new_x < GRID_WIDTH as i32 &&
                   new_y >= 0 && new_y < GRID_HEIGHT as i32 {
                    if self.grid[new_y as usize][new_x as usize].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn get_active_player_ids(&self) -> HashSet<usize> {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "get_active_players");
        self.players.iter().map(|p| p.id).collect()
    }

    pub fn start_attack(&mut self, source: usize, target: usize, investment: i32) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "start_attack");
        if let Some(player) = self.players.iter_mut().find(|p| p.id == source) {
            player.resources -= investment;
            let attack = AttackMovement::new(source, target, investment);
            
            // Handle counter-attacks
            let mut should_add = true;
            let mut movements_to_remove = Vec::new();
            
            for (i, existing_attack) in self.attack_movements.iter_mut().enumerate() {
                if existing_attack.source == target && existing_attack.target == source {
                    // Counter-attack scenario
                    let min_investment = std::cmp::min(attack.investment, existing_attack.investment);
                    
                    // Reduce both investments
                    existing_attack.investment -= min_investment;
                    let mut new_attack = attack.clone();
                    new_attack.investment -= min_investment;
                    
                    // Check if existing attack should be removed
                    if existing_attack.investment <= 0 {
                        movements_to_remove.push(i);
                    }
                    
                    // Check if new attack should still be added
                    if new_attack.investment <= 0 {
                        should_add = false;
                    } else {
                        // Update attack with reduced investment
                        should_add = true;
                    }
                    
                    break;
                }
            }
            
            // Remove any depleted attacks
            for &index in movements_to_remove.iter().rev() {
                self.attack_movements.remove(index);
            }
            
            // Add the new attack if it still has investment
            if should_add {
                self.attack_movements.push(attack);
            }
        }
    }

    pub fn eliminate_player(&mut self, player_id: usize) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "eliminate_player");
        // Remove player's territory
        for row in &mut self.grid {
            for cell in row {
                if let Some(id) = cell {
                    if *id == player_id {
                        *cell = None;
                    }
                }
            }
        }

        // Remove related attack movements
        self.attack_movements.retain(|attack| {
            attack.source != player_id && attack.target != player_id
        });
    }

    pub fn update_grid(&mut self) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "update_grid");
        let active_players = self.get_active_player_ids();
        for row in &mut self.grid {
            for cell in row {
                if let Some(id) = cell {
                    if !active_players.contains(id) {
                        *cell = None;
                    }
                }
            }
        }
    }
}
