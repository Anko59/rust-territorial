mod modules;
mod web;

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{self, Duration};
use warp::Filter;

use modules::config::*;
use modules::game::GameState;
use modules::timing::{TimingStats, start_timing_logger};

lazy_static::lazy_static! {
    pub static ref TIMING_STATS: Arc<RwLock<TimingStats>> = Arc::new(RwLock::new(TimingStats::new()));
}

#[tokio::main]
async fn main() {
    println!("Initializing game server...");
    
    let game_state = Arc::new(RwLock::new(GameState::new()));
    game_state.write().await.initialize_players();

    // Start timing logger
    tokio::spawn(start_timing_logger(TIMING_STATS.clone()));

    // Create broadcast channel with large capacity
    let (tx, _rx) = tokio::sync::broadcast::channel::<String>(8192);
    let tx = Arc::new(tx);

    // Keep a receiver around to prevent channel from closing
    let _keep_alive_rx = tx.subscribe();

    // Set up periodic game state updates
    let game_state_update = game_state.clone();
    let tx_update = tx.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(UPDATE_INTERVAL_MS));
        loop {
            interval.tick().await;
            {
                let _timer = modules::timing::ExecutionTimer::new(TIMING_STATS.clone(), "game_state_update");
                let mut state = game_state_update.write().await;
                state.update();
                
                // Broadcast updated state
                if let Ok(state_json) = serde_json::to_string(&state.grid) {
                    let _ = tx_update.send(state_json);
                }
            }

            // Small delay to prevent tight loops
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    });

    let game_state_ws = game_state.clone();
    let tx_ws = tx.clone();

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let game_state = game_state_ws.clone();
            let tx = tx_ws.clone();
            ws.on_upgrade(move |socket| web::handle_websocket(socket, game_state, tx))
        });

    // Serve static files from the web directory
    let content_route = warp::fs::dir("src/web");

    let routes = content_route.or(ws_route);

    println!("Server configuration:");
    println!("  - Grid size: {}x{}", GRID_WIDTH, GRID_HEIGHT);
    println!("  - Number of players: {}", NUM_PLAYERS);
    println!("  - Update interval: {}ms", UPDATE_INTERVAL_MS);
    println!("  - Performance monitoring interval: 60s");
    println!("\nServer starting on http://localhost:3030");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
