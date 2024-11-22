use std::sync::Arc;
use tokio::sync::RwLock;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use tokio_stream::wrappers::BroadcastStream;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use crate::modules::game::GameState;
use crate::modules::timing::ExecutionTimer;
use crate::TIMING_STATS;

pub async fn handle_websocket(
    ws: WebSocket,
    game_state: Arc<RwLock<GameState>>,
    tx: Arc<broadcast::Sender<String>>
) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    // Create a new subscriber and keep the receiver around
    let rx = tx.subscribe();
    let mut rx_stream = BroadcastStream::new(rx);

    // Increased channel capacity to prevent backpressure
    let (msg_tx, mut msg_rx) = mpsc::channel(1024);

    // Send initial state
    {
        let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "send_initial_state");
        let state = game_state.read().await;
        let initial_state = serde_json::to_string(&state.grid).unwrap();
        if let Err(_) = ws_tx.send(Message::text(initial_state)).await {
            return;
        }
    }

    // Handle incoming messages in a separate task
    let msg_tx_clone = msg_tx.clone();
    tokio::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "handle_incoming_message");
            match result {
                Ok(msg) => {
                    if msg.is_ping() {
                        if let Err(_) = msg_tx_clone.send(Message::pong(vec![])).await {
                            break;
                        }
                    } else if msg.is_close() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // Send game state updates
    let mut consecutive_errors = 0;
    loop {
        tokio::select! {
            Some(msg) = msg_rx.recv() => {
                let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "send_message");
                if let Err(_) = ws_tx.send(msg).await {
                    consecutive_errors += 1;
                    if consecutive_errors > 3 {
                        break;
                    }
                } else {
                    consecutive_errors = 0;
                }
            }
            Some(Ok(state_msg)) = rx_stream.next() => {
                let _timer = ExecutionTimer::new(TIMING_STATS.clone(), "broadcast_state");
                
                // Send a ping to check connection health
                if let Err(_) = ws_tx.send(Message::ping(vec![])).await {
                    break;
                }

                // Small delay to ensure ping is processed
                sleep(Duration::from_millis(10)).await;

                if let Err(_) = ws_tx.send(Message::text(state_msg)).await {
                    consecutive_errors += 1;
                    if consecutive_errors > 3 {
                        break;
                    }
                } else {
                    consecutive_errors = 0;
                }
            }
            else => break,
        }

        sleep(Duration::from_millis(5)).await;
    }
    
    let _ = ws_tx.send(Message::close()).await;
}
