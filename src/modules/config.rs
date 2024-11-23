// Grid configuration
pub const GRID_WIDTH: usize = 800;
pub const GRID_HEIGHT: usize = 600;
pub const NUM_PLAYERS: usize = 20;
pub const MIN_AREA_THRESHOLD: i32 = 1;

// Map configuration
pub const MAP_WIDTH: usize = 800;
pub const MAP_HEIGHT: usize = 600;
pub const MAX_TEMPERATURE: f64 = 40.0;  // Maximum temperature at equator
pub const MIN_TEMPERATURE: f64 = -15.0;  // Minimum temperature at poles
pub const TEMPERATURE_LAPSE_RATE: f64 = 0.006;  // Temperature decrease per meter of elevation
pub const MOUNTAIN_ELEVATION: f64 = 3000.0;  // Elevation threshold for mountains
pub const ACCESSIBILITY_ELEVATION: f64 = 2000.0;  // Maximum elevation for accessible terrain
pub const MAP_VERSION: &str = "1.0.0";  // Used for cache invalidation

// Map file paths
pub const ELEVATION_MAP_PATH: &str = "src/map_files/world_elevation.pkl";
pub const RAINFALL_MAP_PATH: &str = "src/map_files/world_rainfall.pkl";
pub const LAT_MAP_PATH: &str = "src/map_files/world_lat.pkl";
pub const LON_MAP_PATH: &str = "src/map_files/world_lon.pkl";

// Player configuration
pub const BASE_INTEREST_RATE: f64 = 0.01;  // 1% base interest rate
pub const MAX_RESOURCES_MULTIPLIER: i32 = 100;  // Resource cap multiplier
pub const MIN_EXPANSION_COST: i32 = 5;

// Game update configuration
pub const UPDATE_INTERVAL_MS: u64 = 100;  // Overall game speed control
pub const BROADCAST_INTERVAL_MS: u64 = 500;  // Send updates to clients every 500ms
pub const ATTACK_MOVEMENT_UPDATE_INTERVAL: u64 = 1;   // Update every tick
pub const PLAYER_EXPANSION_UPDATE_INTERVAL: u64 = 5;  // Every 5 ticks
pub const PLAYER_RESOURCE_UPDATE_INTERVAL: u64 = 5;   // Every 2 ticks
pub const BONUS_UPDATE_INTERVAL: u64 = 50;           // Every 50 resource updates (250 ticks)
pub const AREA_UPDATE_INTERVAL: u64 = 3;             // Every 3 ticks
pub const NEIGHBOR_UPDATE_INTERVAL: u64 = 10;        // Every 10 ticks
pub const CENTER_OF_MASS_UPDATE_INTERVAL: u64 = 20;   // Update center of mass every 20 ticks
pub const PLAYER_INFO_BROADCAST_MS: u64 = 1000;       // Broadcast player info every 1000ms

// Grid sampling configuration
pub const GRID_REDUCTION_FACTOR: usize = 2;  // Sample every 2nd cell when finding neighbors
