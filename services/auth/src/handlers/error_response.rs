// services/auth/src/handlers/error_response.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::common::error::DomainError;
use serde_json::json;

/// Application error wrapper that can be converted to HTTP responses
pub struct AppError(DomainError);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            DomainError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            DomainError::AlreadyExists(msg) => (StatusCode::CONFLICT, msg),
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            DomainError::InfraError(msg) => {
                tracing::error!("Infrastructure error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        AppError(err)
    }
}
