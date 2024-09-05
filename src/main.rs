mod app;
mod websocket;
mod whiteboard;

#[tokio::main]
async fn main() {
    let routes = app::routes();

    println!("Server starting on http://0.0.0.0:7860");
    warp::serve(routes).run(([0, 0, 0, 0], 7860)).await;
}
