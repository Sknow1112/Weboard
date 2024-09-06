mod app;
mod whiteboard;
mod websocket;

use warp::Filter;

#[tokio::main]
async fn main() {
    let whiteboard = whiteboard::Whiteboard::new();
    let whiteboard = std::sync::Arc::new(tokio::sync::Mutex::new(whiteboard));

    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || whiteboard.clone()))
        .and_then(websocket::ws_handler);

    let static_files = warp::fs::dir("static");
    let index = warp::get().and(warp::path::end()).and(warp::fs::file("static/index.html"));

    let routes = websocket_route
        .or(static_files)
        .or(index);

    println!("Server starting on http://0.0.0.0:7860");
    warp::serve(routes).run(([0, 0, 0, 0], 7860)).await;
}
