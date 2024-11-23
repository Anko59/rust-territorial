use serde::{Serialize, Serializer};
use crate::modules::config::*;
use crate::modules::types::{Grid, Players, AttackMovement, WorldMap, GridCell};
use crate::modules::timing::ExecutionTimer;
use crate::TIMING_STATS;
use super::{grid, neighbors::NeighborPair};

#[derive(Clone)]
pub struct UpdateTimers {
    pub last_attack_update: u64,
    pub last_expansion_update: u64,
    pub last_resource_update: u64,
    pub last_area_update: u64,
    pub last_neighbor_update: u64,
    pub last_center_of_mass_update: u64,
}

impl Default for UpdateTimers {
    fn default() -> Self {
        Self {
            last_attack_update: 0,
            last_expansion_update: 0,
            last_resource_update: 0,
            last_area_update: 0,
            last_neighbor_update: 0,
            last_center_of_mass_update: 0,
        }
    }
}

#[derive(Clone)]
pub struct GameState {
    pub grid: Grid,
    pub players: Players,
    pub world_map: WorldMap,
    pub(crate) attack_movements: Vec<AttackMovement>,
    pub(crate) update_timers: Option<UpdateTimers>,
    pub(crate) neighbor_pairs: Vec<NeighborPair>,
    pub(crate) update_count: u64,
}

impl Serialize for GameState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("GameState", 4)?;
        state.serialize_field("players", &self.players)?;
        state.serialize_field("grid", &grid::serialize_grid(&self.grid))?;
        state.serialize_field("world_map", &self.world_map)?;
        state.serialize_field("attack_movements", &self.attack_movements)?;
        state.end()
    }
}

impl GameState {
    pub fn new() -> Self {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "game_state_creation");
        let grid = vec![vec![GridCell::Available; GRID_WIDTH]; GRID_HEIGHT];
        let players = Vec::new();
        let world_map = WorldMap::new(MAP_WIDTH, MAP_HEIGHT);
        let attack_movements = Vec::new();
        let update_timers = Some(UpdateTimers::default());
        let neighbor_pairs = Vec::new();
        let update_count = 0;
        
        GameState { 
            grid, 
            players, 
            world_map,
            attack_movements, 
            update_timers,
            neighbor_pairs,
            update_count,
        }
    }

    pub fn is_position_available(&self, x: i32, y: i32, radius: i32) -> bool {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "check_position_availability");

        // Check if position is already occupied
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0 && new_x < GRID_WIDTH as i32 &&
                   new_y >= 0 && new_y < GRID_HEIGHT as i32 {
                    if self.grid[new_y as usize][new_x as usize] != GridCell::Available {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn start_attack(&mut self, source: i32, target: i32, investment: i32) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "start_attack");
        if let Some(player) = self.players.iter_mut().find(|p| p.id == source) {
            player.resources -= investment;
            let attack = AttackMovement::new(source, target, investment);
            
            let mut should_add = true;
            let mut movements_to_remove = Vec::new();
            
            for (i, existing_attack) in self.attack_movements.iter_mut().enumerate() {
                if existing_attack.source == target && existing_attack.target == source {
                    let min_investment = std::cmp::min(attack.investment, existing_attack.investment);
                    existing_attack.investment -= min_investment;
                    let mut new_attack = attack.clone();
                    new_attack.investment -= min_investment;
                    
                    if existing_attack.investment <= 0 {
                        movements_to_remove.push(i);
                    }
                    
                    if new_attack.investment <= 0 {
                        should_add = false;
                    } else {
                        should_add = true;
                    }
                    
                    break;
                }
            }
            
            for &index in movements_to_remove.iter().rev() {
                self.attack_movements.remove(index);
            }
            
            if should_add {
                self.attack_movements.push(attack);
            }
        }
    }

    pub fn eliminate_player(&mut self, player_id: i32) {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "player_elimination");
        let grid_cell = GridCell::from_i32(player_id);
        for row in &mut self.grid {
            for cell in row {
                
                if *cell == grid_cell {
                    *cell = GridCell::Available;
                }
            }
        }

        self.attack_movements.retain(|attack| {
            attack.source != player_id && attack.target != player_id
        });
    }
}
