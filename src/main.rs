mod app;
mod websocket;
mod whiteboard;
mod database;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Initialize the database
    let db = match database::init_db() {
        Ok(db) => Arc::new(db),
        Err(e) => {
            log::error!("Failed to initialize database: {}", e);
            return;
        }
    };

    // Create the routes
    let routes = app::routes(db);

    // Start the server
    log::info!("Server starting on http://0.0.0.0:7860");
    warp::serve(routes).run(([0, 0, 0, 0], 7860)).await;
}
