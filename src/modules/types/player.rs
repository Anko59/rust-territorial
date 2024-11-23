use serde::Serialize;
use crate::modules::config::*;
use rand::Rng;


#[derive(Clone, Serialize, Debug)]
pub struct Player {
    pub id: i32,
    pub x: usize,
    pub y: usize,
    pub resources: i32,
    pub area: i32,
    pub base_interest_rate: f64,
    pub name: String,
    pub center_x: f64,
    pub center_y: f64,
    pub average_land_value: f64,
}

#[derive(Serialize)]
pub struct PlayerInfo {
    pub id: i32,
    pub name: String,
    pub resources: i32,
    pub center_x: f64,
    pub center_y: f64,
}

impl Player {
    pub fn new(id: i32, x: usize, y: usize) -> Self {
        println!("Creating new player {} at position ({}, {})", id, x, y);
        Player {
            id,
            x,
            y,
            resources: 1000,
            area: 1,
            base_interest_rate: BASE_INTEREST_RATE,
            name: format!("Player {}", id),
            center_x: x as f64,
            center_y: y as f64,
            average_land_value: 1.0,
        }
    }

    pub fn to_info(&self) -> PlayerInfo {
        PlayerInfo {
            id: self.id,
            name: self.name.clone(),
            resources: self.resources,
            center_x: self.center_x,
            center_y: self.center_y,
        }
    }

    #[inline]
    pub fn max_resources(&self) -> i32 {
        std::cmp::max(self.area * MAX_RESOURCES_MULTIPLIER, 2000)
    }

    #[inline]
    pub fn interest_rate(&self) -> f64 {
        let max_resources = self.max_resources();
        let resource_factor = f64::max(1.0 - (self.resources as f64 / max_resources as f64).powi(2), 0.0);
        self.base_interest_rate * resource_factor
    }

    pub fn update_resources(&mut self) {
        // Regular interest-based growth
        self.resources = std::cmp::min(
            ((self.resources as f64 * (1.0 + self.interest_rate())) as i32) + 1,
            self.max_resources()
        );
    }

    pub fn try_expand(&self) -> bool {
        self.resources >= MIN_EXPANSION_COST
    }

    pub fn get_target(&self, neighbors: &[usize]) -> Option<(usize, i32)> {
        if neighbors.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        if !rng.gen_bool(0.3) {
            return None;
        }

        let target = neighbors[rng.gen_range(0..neighbors.len())];
        let investment_ratio = rng.gen_range(0.01..0.3);
        let investment = (self.resources as f64 * investment_ratio) as i32;
        
        if investment >= MIN_EXPANSION_COST {
            Some((target, investment))
        } else {
            None
        }
    }
}
