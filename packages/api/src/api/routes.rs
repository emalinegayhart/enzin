use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use crate::engine::IndexManager;
use super::handlers;

pub fn create_routes(manager: Arc<IndexManager>) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/indexes", post(handlers::create_index))
        .route("/indexes", get(handlers::list_indexes))
        .route("/indexes/:name", delete(handlers::delete_index))
        .route("/indexes/:name/documents", post(handlers::index_documents))
        .route("/indexes/:name/search", get(handlers::search))
        .with_state(manager)
}
