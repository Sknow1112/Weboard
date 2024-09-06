use crate::app::{ClientMessage, ServerMessage};
use crate::whiteboard::Whiteboard;
use futures::{FutureExt, SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};

pub async fn ws_handler(
    ws: warp::ws::Ws,
    whiteboard: Arc<Mutex<Whiteboard>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(ws.on_upgrade(move |socket| client_connection(socket, whiteboard)))
}

async fn client_connection(ws: WebSocket, whiteboard: Arc<Mutex<Whiteboard>>) {
    let (mut client_ws_sender, mut client_ws_rcv) = ws.split();

    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(69));

    loop {
        tokio::select! {
            msg = client_ws_rcv.next().fuse() => {
                match msg {
                    Some(Ok(msg)) => {
                        if let Ok(text) = msg.to_str() {
                            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text) {
                                let mut wb = whiteboard.lock().await;
                                match client_msg {
                                    ClientMessage::Draw(action) => {
                                        wb.add_action(action);
                                    }
                                    ClientMessage::Clear => {
                                        wb.clear();
                                    }
                                }
                            }
                        }
                    }
                    _ => break,
                }
            }
            _ = interval.tick() => {
                let wb = whiteboard.lock().await;
                let actions = wb.get_actions();
                let server_msg = ServerMessage::Update(actions);
                let msg = serde_json::to_string(&server_msg).unwrap();
                if let Err(_) = client_ws_sender.send(Message::text(msg)).await {
                    break;
                }
            }
        }
    }
}
