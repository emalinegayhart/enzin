use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;
use crate::engine::IndexManager;

#[derive(Deserialize)]
pub struct CreateIndexRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateIndexResponse {
    pub name: String,
    pub created_at: String,
}

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn create_index(
    State(manager): State<Arc<IndexManager>>,
    Json(req): Json<CreateIndexRequest>,
) -> Result<(StatusCode, Json<CreateIndexResponse>), crate::error::EnzinError> {
    manager.create_index(&req.name).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateIndexResponse {
            name: req.name,
            created_at: chrono::Utc::now().to_rfc3339(),
        }),
    ))
}

#[derive(Serialize)]
pub struct ListIndexesResponse {
    pub indexes: Vec<String>,
}

pub async fn list_indexes(
    State(manager): State<Arc<IndexManager>>,
) -> Json<ListIndexesResponse> {
    let indexes = manager.list_indexes().await;
    Json(ListIndexesResponse { indexes })
}

#[derive(Serialize)]
pub struct DeleteIndexResponse {
    pub deleted: String,
}

pub async fn delete_index(
    State(manager): State<Arc<IndexManager>>,
    Path(name): Path<String>,
) -> Result<Json<DeleteIndexResponse>, crate::error::EnzinError> {
    manager.delete_index(&name).await?;
    Ok(Json(DeleteIndexResponse { deleted: name }))
}

#[derive(Serialize)]
pub struct IndexDocumentsResponse {
    pub indexed: usize,
}

pub async fn index_documents(
    State(manager): State<Arc<IndexManager>>,
    Path(name): Path<String>,
    body: String,
) -> Result<(StatusCode, Json<IndexDocumentsResponse>), crate::error::EnzinError> {
    let parsed: Value = serde_json::from_str(&body)
        .map_err(|e| crate::error::EnzinError::InvalidDocument(format!("invalid json: {}", e)))?;

    let documents = match parsed {
        Value::Array(arr) => arr,
        Value::Object(_) => vec![parsed],
        _ => {
            return Err(crate::error::EnzinError::InvalidDocument(
                "expected object or array of objects".to_string(),
            ))
        }
    };

    let count = manager.index_documents(&name, documents).await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(IndexDocumentsResponse { indexed: count }),
    ))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default)]
    pub fuzzy: bool,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    20
}

#[derive(Serialize)]
pub struct SearchHit {
    #[serde(flatten)]
    pub document: Value,
    #[serde(rename = "_score")]
    pub score: f32,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub hits: Vec<SearchHit>,
    pub total: usize,
    pub took_ms: u128,
}

pub async fn search(
    State(manager): State<Arc<IndexManager>>,
    Path(name): Path<String>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, crate::error::EnzinError> {
    let start = Instant::now();
    
    let (results, total) = manager
        .search_with_options(&name, &params.q, params.fuzzy, params.limit, params.offset)
        .await?;

    let took_ms = start.elapsed().as_millis();

    let hits = results
        .into_iter()
        .map(|result| SearchHit {
            document: result.document,
            score: result.score,
        })
        .collect();

    Ok(Json(SearchResponse {
        query: params.q,
        hits,
        total,
        took_ms,
    }))
}
