use axum::{
    routing::get,
    Json, Router,
};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let listener = TcpListener::bind("127.0.0.1:7700")
        .await
        .expect("Failed to bind to port 7700");

    println!("Server running on http://127.0.0.1:7700");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
