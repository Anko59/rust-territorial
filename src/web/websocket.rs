use std::sync::Arc;
use futures::{StreamExt, SinkExt};
use tokio::sync::{broadcast, RwLock};
use warp::ws::{Message, WebSocket};
use serde_json::json;

use crate::modules::timing::ExecutionTimer;
use crate::TIMING_STATS;
use crate::modules::game::{GameState, grid};

pub async fn handle_websocket(
    ws: WebSocket, 
    game_state: Arc<RwLock<GameState>>, 
    tx: Arc<broadcast::Sender<String>>,
    player_info_tx: Arc<broadcast::Sender<String>>,
) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();
    let mut player_info_rx = player_info_tx.subscribe();

    // Send initial state
    {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "send_initial_state");
        
        // Clone state data under lock then release quickly
        let (grid_data, players, world_map, attack_movements) = {
            let state = game_state.read().await;
            (
                grid::serialize_grid(&state.grid),
                state.players.clone(),
                state.world_map.clone(),
                state.attack_movements.clone(),
            )
        };

        // Create state message outside the lock
        let state_json = json!({
            "type": "game_state",
            "data": {
                "grid": grid_data,
                "players": players,
                "world_map": {
                    "color_map": world_map.color_map,
                    "traversability_map": world_map.traversability_map,
                    "livability_map": world_map.livability_map,
                },
                "attack_movements": attack_movements,
            }
        });

        let _ = ws_tx.send(Message::text(serde_json::to_string(&state_json).unwrap())).await;

        // Send initial player info
        let player_infos: Vec<_> = players.iter().map(|p| p.to_info()).collect();
        let info_json = json!({
            "type": "player_info",
            "data": player_infos
        });
        let _ = ws_tx.send(Message::text(serde_json::to_string(&info_json).unwrap())).await;
    }

    // Forward broadcast messages to this client
    let mut forward = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(msg) = rx.recv() => {
                    let _ = ws_tx.send(Message::text(msg)).await;
                }
                Ok(msg) = player_info_rx.recv() => {
                    let _ = ws_tx.send(Message::text(msg)).await;
                }
            }
        }
    });

    // Process incoming messages
    let mut receive = tokio::spawn(async move {
        while let Some(Ok(_msg)) = ws_rx.next().await {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "client_message_handling");
            // Currently ignoring client messages
            // In the future, we could handle client input here
        }
    });

    // Run both tasks concurrently
    tokio::select! {
        _ = &mut forward => {},
        _ = &mut receive => {},
    }
}
