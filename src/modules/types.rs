use std::collections::HashSet;
use rand::Rng;
use serde::Serialize;
use crate::modules::config::*;

pub type Grid = Vec<Vec<Option<usize>>>;
pub type Players = Vec<Player>;

#[derive(Clone, Copy, Serialize, Debug)]
pub struct Player {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub resources: i32,
    pub area: i32,
    pub base_interest_rate: f64,
}

impl Player {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        println!("Creating new player {} at position ({}, {})", id, x, y);
        Player {
            id,
            x,
            y,
            resources: 1000,
            area: 1,
            base_interest_rate: BASE_INTEREST_RATE,
        }
    }

    #[inline]
    pub fn max_resources(&self) -> i32 {
        std::cmp::max(self.area * MAX_RESOURCES_MULTIPLIER, 2000)
    }

    #[inline]
    pub fn interest_rate(&self) -> f64 {
        let resource_ratio = self.resources as f64 / self.max_resources() as f64;
        let resource_factor = f64::max(1.0 - resource_ratio.powi(2), 0.0);
        self.base_interest_rate * resource_factor
    }

    pub fn update_resources(&mut self) {
        let territory_resources = self.area;
        let interest = (self.resources as f64 * self.interest_rate()) as i32;
        let total_gain = territory_resources + interest;        
        self.resources = std::cmp::min(
            self.resources + total_gain,
            self.max_resources()
        );
    }

    pub fn try_expand(&self) -> bool {
        let resource_ratio = self.resources as f64 / self.max_resources() as f64;
        let expansion_chance = BASE_EXPANSION_CHANCE * resource_ratio;
        let will_expand = rand::thread_rng().gen_bool(expansion_chance);
        
        
        will_expand
    }

    pub fn calculate_expansion_investment(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let investment_ratio = rng.gen_range(0.2..0.4);
        let base_investment = (self.resources as f64 * investment_ratio) as i32;
        let investment = std::cmp::max(base_investment, MIN_EXPANSION_COST);
        let final_investment = std::cmp::min(investment, self.resources);
        
        final_investment
    }
}

#[derive(Clone, Serialize)]
pub struct AttackMovement {
    pub source: usize,
    pub target: usize,
    pub investment: i32,
    pub border_pixels: Vec<(usize, usize)>,
    pub is_started: bool,
    cached_grid_size: Option<(usize, usize)>,
}

impl AttackMovement {
    pub fn new(source: usize, target: usize, investment: i32) -> Self {
        AttackMovement {
            source,
            target,
            investment,
            border_pixels: Vec::with_capacity(investment as usize),
            is_started: false,
            cached_grid_size: None,
        }
    }

    pub fn start(&mut self, grid: &Grid) {
        self.cache_grid_size(grid);
        self.border_pixels = self.find_start_pixels(grid);
        self.is_started = true;
    }

    #[inline]
    fn cache_grid_size(&mut self, grid: &Grid) {
        if self.cached_grid_size.is_none() {
            self.cached_grid_size = Some((grid[0].len(), grid.len()));
        }
    }

    fn find_start_pixels(&self, grid: &Grid) -> Vec<(usize, usize)> {
        let (grid_width, grid_height) = self.cached_grid_size.unwrap_or((grid[0].len(), grid.len()));
        let mut pixels = Vec::with_capacity(grid_width.min(grid_height));
        let mut source_pixels = Vec::with_capacity(grid_width);
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        for y in 0..grid_height {
            source_pixels.clear();
            let row = &grid[y];
            
            for x in 0..grid_width {
                if let Some(id) = row[x] {
                    if id == self.source {
                        source_pixels.push(x);
                    }
                }
            }
            
            for &x in &source_pixels {
                for &(dx, dy) in &DIRECTIONS {
                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;
                    
                    if new_x >= 0 && new_x < grid_width as i32 &&
                       new_y >= 0 && new_y < grid_height as i32 {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        
                        match grid[new_y][new_x] {
                            Some(id) if id == self.target => pixels.push((new_x, new_y)),
                            None if self.target == usize::MAX => pixels.push((new_x, new_y)),
                            _ => {}
                        }
                    }
                }
            }
        }
        
        pixels
    }

    pub fn get_next_pixels(&self, grid: &Grid) -> Vec<(usize, usize)> {
        let (grid_width, grid_height) = self.cached_grid_size.unwrap_or((grid[0].len(), grid.len()));
        let mut next_pixels = HashSet::with_capacity(self.border_pixels.len() * 5); // Increased capacity for center position
        const DIRECTIONS: [(i32, i32); 5] = [(0, 0), (0, 1), (1, 0), (0, -1), (-1, 0)]; // Added (0, 0) for center position
        
        for chunk in self.border_pixels.chunks(64) {
            for &(x, y) in chunk {
                for &(dx, dy) in &DIRECTIONS {
                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;
                    
                    if new_x >= 0 && new_x < grid_width as i32 &&
                       new_y >= 0 && new_y < grid_height as i32 {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        
                        match grid[new_y][new_x] {
                            Some(id) if id == self.target => next_pixels.insert((new_x, new_y)),
                            None if self.target == usize::MAX => next_pixels.insert((new_x, new_y)),
                            _ => false
                        };
                    }
                }
            }
        }

        let result = next_pixels.into_iter().collect::<Vec<_>>();
        result
    }
}
