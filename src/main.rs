mod api;
mod engine;
mod error;

use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("ENZIN_PORT")
        .unwrap_or_else(|_| "7700".to_string())
        .parse::<u16>()
        .expect("ENZIN_PORT must be a valid u16");

    let data_dir = env::var("ENZIN_DATA_DIR")
        .unwrap_or_else(|_| "./data".to_string());
    let data_dir = PathBuf::from(&data_dir);
    
    std::fs::create_dir_all(&data_dir).expect("Failed to create data directory");

    let manager = Arc::new(engine::IndexManager::new(data_dir));
    let app = api::create_routes(manager);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {}", addr));

    println!("Server running on http://0.0.0.0:{}", port);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
