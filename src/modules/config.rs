// Grid configuration
pub const GRID_WIDTH: usize = 800;
pub const GRID_HEIGHT: usize = 600;
pub const NUM_PLAYERS: usize = 20;
pub const MIN_AREA_THRESHOLD: i32 = 1;

// Player configuration
pub const BASE_INTEREST_RATE: f64 = 0.05;  // Increased from 0.01 for faster resource gain
pub const MAX_RESOURCES_MULTIPLIER: i32 = 50;  // Decreased from 100 to encourage spending
pub const MIN_EXPANSION_COST: i32 = 5;  // Decreased from 10 for more frequent expansions
pub const BASE_EXPANSION_CHANCE: f64 = 0.8;  // Increased from 0.5 for more aggressive expansion

// Game update configuration
pub const UPDATE_INTERVAL_MS: u64 = 100;  // Decreased from 1000 for faster gameplay
