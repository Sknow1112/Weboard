use warp::Filter;
use crate::websocket;
use crate::database::Database;
use std::sync::Arc;
use crate::websocket::WhiteboardState;

pub fn routes(db: Arc<Database>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || Arc::clone(&db)))
        .map(|ws: warp::ws::Ws, db: Arc<Database>| {
            ws.on_upgrade(move |socket| websocket::handle_connection(socket, WhiteboardState::new(db)))
        });

    let static_files = warp::path("static").and(warp::fs::dir("static"));
    let index = warp::path::end().and(warp::fs::file("static/index.html"));

    websocket_route.or(static_files).or(index)
}
