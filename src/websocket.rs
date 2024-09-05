use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use crate::whiteboard::{WhiteboardAction, WhiteboardManager, DrawAction};
use crate::database::Database;
use serde_json::Value;

pub struct WhiteboardState {
    tx: broadcast::Sender<WhiteboardAction>,
    manager: Arc<Mutex<WhiteboardManager>>,
    db: Arc<Database>,
}

impl WhiteboardState {
    pub fn new(db: Arc<Database>) -> Arc<Self> {
        let (tx, _) = broadcast::channel(100);
        let manager = Arc::new(Mutex::new(WhiteboardManager::new(db.clone())));
        Arc::new(WhiteboardState { tx, manager, db })
    }
}

pub async fn handle_connection(ws: WebSocket, state: Arc<WhiteboardState>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = state.tx.subscribe();

    log::info!("New WebSocket connection established");

    // Send initial state to the client
    let initial_state = state.manager.lock().unwrap().get_current_state();
    let initial_state_msg = serde_json::to_string(&WhiteboardAction::InitialState(initial_state)).unwrap();
    if let Err(e) = ws_tx.send(Message::text(initial_state_msg)).await {
        log::error!("Failed to send initial state: {}", e);
        return;
    }
    log::info!("Sent initial state to client");

    // Handle incoming messages
    let state_clone = Arc::clone(&state);
    tokio::task::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            match result {
                Ok(msg) => {
                    if let Ok(text) = msg.to_str() {
                        match serde_json::from_str::<Value>(text) {
                            Ok(value) => {
                                if let Some(action_type) = value.get("type") {
                                    match action_type.as_str() {
                                        Some("Draw") => {
                                            if let Ok(draw_action) = serde_json::from_value::<DrawAction>(value) {
                                                let action = WhiteboardAction::Draw(draw_action);
                                                log::info!("Received action: {:?}", action);
                                                state_clone.manager.lock().unwrap().apply_action(&action);
                                                let _ = state_clone.tx.send(action);
                                            } else {
                                                log::warn!("Failed to parse Draw action: {}", text);
                                            }
                                        },
                                        Some("Clear") => {
                                            let action = WhiteboardAction::Clear;
                                            log::info!("Received action: {:?}", action);
                                            state_clone.manager.lock().unwrap().apply_action(&action);
                                            let _ = state_clone.tx.send(action);
                                        },
                                        Some("Zoom") => {
                                            if let Some(value) = value.get("value") {
                                                if let Some(zoom) = value.as_f64() {
                                                    let action = WhiteboardAction::Zoom(zoom);
                                                    log::info!("Received action: {:?}", action);
                                                    state_clone.manager.lock().unwrap().apply_action(&action);
                                                    let _ = state_clone.tx.send(action);
                                                } else {
                                                    log::warn!("Invalid zoom value: {}", text);
                                                }
                                            } else {
                                                log::warn!("Zoom action missing value: {}", text);
                                            }
                                        },
                                        _ => log::warn!("Unknown action type: {}", text),
                                    }
                                } else {
                                    log::warn!("Message missing 'type' field: {}", text);
                                }
                            },
                            Err(e) => log::warn!("Failed to parse message as JSON: {}, Error: {}", text, e),
                        }
                    }
                }
                Err(e) => log::error!("WebSocket error: {}", e),
            }
        }
        log::info!("WebSocket connection closed");
    });

    // Rest of the function remains the same...
}
