use serde::Serialize;
use crate::modules::types::{Grid, Player};

#[derive(Clone, Serialize, Debug)]
pub struct AttackMovement {
    pub source: i32,
    pub target: i32,
    pub investment: i32,
    pub is_started: bool,
    pub border_pixels: Vec<(usize, usize)>,
}

impl AttackMovement {
    pub fn new(source: i32, target: i32, investment: i32) -> Self {
        AttackMovement {
            source,
            target,
            investment,
            is_started: false,
            border_pixels: Vec::new(),
        }
    }

    pub fn start(&mut self, grid: &Grid) {
        if !self.is_started {
            self.border_pixels = self.find_border_pixels(grid);
            self.is_started = true;
        }
    }

    fn find_border_pixels(&self, grid: &Grid) -> Vec<(usize, usize)> {
        let mut border = Vec::new();
        let height = grid.len();
        let width = grid[0].len();

        for y in 0..height {
            for x in 0..width {
                let id = grid[y][x].to_i32();
                if id == self.source {
                    // Check all adjacent cells
                    for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;
                        
                        if new_x >= 0 && new_x < width as i32 && 
                            new_y >= 0 && new_y < height as i32 {
                            let new_x = new_x as usize;
                            let new_y = new_y as usize;
                            
                            if grid[new_y][new_x].to_i32() == self.target {
                                border.push((x, y));
                                break;
                            }
                        }
                    }
                }
            }
        }
        border
    }

    pub fn get_next_pixels(&self, grid: &Grid) -> Vec<(usize, usize)> {
        let mut next_pixels = Vec::new();
        let height = grid.len();
        let width = grid[0].len();

        for &(x, y) in &self.border_pixels {
            // Check all adjacent cells
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                
                if new_x >= 0 && new_x < width as i32 && 
                   new_y >= 0 && new_y < height as i32 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    
                    if grid[new_y][new_x].to_i32() == self.target {
                        next_pixels.push((new_x, new_y));
                    }
                }
            }
        }
        next_pixels
    }

    pub fn get_next_pixels_costs(&self, next_pixels: &[(usize, usize)], target_player: Option<&Player>, traversability_map: &Vec<Vec<f64>>) -> (i32, i32) {
        let num_pixels = next_pixels.len();
        if num_pixels == 0 {
            return (0, 0);
        }

        // Calculate average traversability score
        let mut total_traversability = 0.0;
        for &(x, y) in next_pixels {
            total_traversability += traversability_map[y][x];
        }
        let traversability_score = total_traversability / num_pixels as f64;

        // If no target (expanding into empty space), cost is based on traversability
        if target_player.is_none() {
            let base_cost = num_pixels as i32;
            let adjusted_cost = (base_cost as f64 * (1.0 + (1.0 - traversability_score))) as i32;
            return (adjusted_cost, num_pixels as i32);
        }

        // Calculate costs for attacking a player
        let target = target_player.unwrap();
        
        // Base cost calculation
        let mut base_cost = num_pixels as f64 * (target.resources as f64 / target.area as f64);
        base_cost *= 1.0 + (1.0 - traversability_score);

        // Adjust cost based on target's resource ratio
        let resource_ratio = target.resources as f64 / (target.max_resources() + 1) as f64;
        let cost_multiplier = 1.0 + resource_ratio;

        // Calculate costs for source and target
        let mut source_cost = (2.0 * base_cost * cost_multiplier) as i32;
        let mut target_cost = (base_cost * cost_multiplier) as i32;

        // Ensure costs don't exceed limits
        source_cost = source_cost.min(self.investment);
        target_cost = target_cost.min(target.resources);

        // Ensure source cost is always twice the target cost
        if source_cost < 2 * target_cost {
            target_cost = source_cost / 2;
        } else if source_cost > 2 * target_cost {
            source_cost = 2 * target_cost;
        }

        // Ensure minimum cost based on number of pixels
        source_cost = source_cost.max(num_pixels as i32);

        (source_cost, target_cost)
    }
}
