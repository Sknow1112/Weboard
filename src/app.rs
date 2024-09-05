use warp::Filter;
use crate::websocket;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || websocket::WhiteboardState::new()))
        .map(|ws: warp::ws::Ws, state| {
            ws.on_upgrade(move |socket| websocket::handle_connection(socket, state))
        });

    let static_files = warp::path("static").and(warp::fs::dir("static"));
    let index = warp::path::end().and(warp::fs::file("static/index.html"));

    websocket_route.or(static_files).or(index)
}
