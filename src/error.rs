use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum EnzinError {
    IndexNotFound(String),
    InvalidDocument(String),
    InternalError(String),
}

impl IntoResponse for EnzinError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            EnzinError::IndexNotFound(msg) => (StatusCode::NOT_FOUND, "index_not_found", msg),
            EnzinError::InvalidDocument(msg) => (StatusCode::BAD_REQUEST, "invalid_document", msg),
            EnzinError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                msg,
            ),
        };

        let body = Json(json!({
            "error": error_type,
            "message": message
        }));

        (status, body).into_response()
    }
}
