use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use crate::whiteboard::WhiteboardAction;

pub struct WhiteboardState {
    tx: broadcast::Sender<WhiteboardAction>,
}

impl WhiteboardState {
    pub fn new() -> Arc<Mutex<Self>> {
        let (tx, _) = broadcast::channel(100);
        Arc::new(Mutex::new(WhiteboardState { tx }))
    }
}

pub async fn handle_connection(ws: WebSocket, state: Arc<Mutex<WhiteboardState>>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = state.lock().unwrap().tx.subscribe();

    // Handle incoming messages
    tokio::task::spawn(async move {
        while let Some(result) = ws_rx.next().await {
            if let Ok(msg) = result {
                if let Ok(text) = msg.to_str() {
                    if let Ok(action) = serde_json::from_str::<WhiteboardAction>(text) {
                        let _ = state.lock().unwrap().tx.send(action);
                    }
                }
            }
        }
    });

    // Send messages to the client
    while let Ok(action) = rx.recv().await {
        if let Ok(msg) = serde_json::to_string(&action) {
            if ws_tx.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    }
}
