mod api;
mod engine;
mod error;

use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let data_dir = PathBuf::from("./data");
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let manager = Arc::new(engine::IndexManager::new(data_dir));
    let app = api::create_routes(manager);

    let listener = TcpListener::bind("127.0.0.1:7700")
        .await
        .expect("Failed to bind to port 7700");

    println!("Server running on http://127.0.0.1:7700");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
