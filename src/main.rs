mod modules;
mod web;

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{self, Duration};
use warp::Filter;
use log::{info, warn};
use clap::Parser;
use serde_json::json;

use modules::config::*;
use modules::game::initialize_game;
use modules::timing::{TimingStats, start_timing_logger};

lazy_static::lazy_static! {
    pub static ref TIMING_STATS: Arc<RwLock<TimingStats>> = Arc::new(RwLock::new(TimingStats::new()));
}

/// Territorial game server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value_t = 3030)]
    port: u16,

    /// Log level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default())
        .filter_level(args.log_level.parse().unwrap_or(log::LevelFilter::Info))
        .init();
    
    info!("Initializing game server...");
    
    let game_state = Arc::new(RwLock::new(initialize_game()));

    // Start timing logger
    tokio::spawn(start_timing_logger(TIMING_STATS.clone()));

    // Create broadcast channels with large capacity
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(8192);
    let tx = Arc::new(tx);
    let (player_info_tx, _player_info_rx) = tokio::sync::broadcast::channel::<String>(8192);
    let player_info_tx = Arc::new(player_info_tx);

    // Keep receivers around to prevent channels from closing
    let _keep_alive_rx = tx.subscribe();
    let _keep_alive_player_info_rx = player_info_tx.subscribe();

    // Set up periodic game state updates
    let game_state_update = game_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(UPDATE_INTERVAL_MS));
        loop {
            interval.tick().await;
            
            // Update game state
            {
                let _timer = modules::timing::ExecutionTimer::new(TIMING_STATS.clone(), "game_state_update");
                let mut state = game_state_update.write().await;
                state.update();
            }
        }
    });

    // Set up periodic state broadcasts
    let game_state_broadcast = game_state.clone();
    let tx_broadcast = tx.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(BROADCAST_INTERVAL_MS));
        loop {
            interval.tick().await;
            
            // Broadcast state
            {
                let _timer = modules::timing::ExecutionTimer::new(TIMING_STATS.clone(), "state_broadcast");
                
                // Clone state data under lock
                let state_clone = {
                    let state = game_state_broadcast.read().await;
                    state.clone()
                };

                // Create state message outside the lock
                let state_json = json!({
                    "type": "game_state",
                    "data": state_clone
                });

                if let Ok(state_str) = serde_json::to_string(&state_json) {
                    if let Err(e) = tx_broadcast.send(state_str) {
                        warn!("Failed to broadcast state: {}", e);
                    }
                }
            }
        }
    });

    // Set up periodic player info broadcasts
    let game_state_player_info = game_state.clone();
    let player_info_tx_broadcast = player_info_tx.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(PLAYER_INFO_BROADCAST_MS));
        loop {
            interval.tick().await;
            
            // Broadcast player info
            {
                let _timer = modules::timing::ExecutionTimer::new(TIMING_STATS.clone(), "player_info_broadcast");
                // Clone players under lock then release quickly
                let player_infos = {
                    let state = game_state_player_info.read().await;
                    state.players.iter().map(|p| p.to_info()).collect::<Vec<_>>()
                };
                // Serialize outside the lock
                let info_json = json!({
                    "type": "player_info",
                    "data": player_infos
                });
                if let Ok(info_str) = serde_json::to_string(&info_json) {
                    if let Err(e) = player_info_tx_broadcast.send(info_str) {
                        warn!("Failed to broadcast player info: {}", e);
                    }
                }
            }
        }
    });

    let game_state_ws = game_state.clone();
    let tx_ws = tx.clone();
    let player_info_tx_ws = player_info_tx.clone();

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let game_state = game_state_ws.clone();
            let tx = tx_ws.clone();
            let player_info_tx = player_info_tx_ws.clone();
            ws.on_upgrade(move |socket| web::handle_websocket(socket, game_state, tx, player_info_tx))
        });

    // Serve static files from the dist directory
    let static_files = warp::fs::dir("src/web/dist");
    
    // Fallback route for SPA - serve index.html for all unmatched routes
    let index_fallback = warp::any()
        .and(warp::fs::file("src/web/dist/index.html"));

    // Combine routes with proper order
    let routes = ws_route
        .or(static_files)
        .or(index_fallback);

    info!("Server configuration:");
    info!("  - Grid size: {}x{}", GRID_WIDTH, GRID_HEIGHT);
    info!("  - Number of players: {}", NUM_PLAYERS);
    info!("  - Update interval: {}ms", UPDATE_INTERVAL_MS);
    info!("  - Broadcast interval: {}ms", BROADCAST_INTERVAL_MS);
    info!("  - Player info broadcast interval: {}ms", PLAYER_INFO_BROADCAST_MS);
    info!("  - Performance monitoring interval: 60s");
    info!("  - Log level: {}", args.log_level);
    info!("\nServer starting on http://localhost:{}", args.port);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], args.port))
        .await;
}
