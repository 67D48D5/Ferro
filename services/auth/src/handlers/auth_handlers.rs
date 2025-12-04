// services/auth/src/handlers/auth_handlers.rs

use application::users::{
    LoginUserRequest, LoginUserUseCase, RegisterUserRequest, RegisterUserUseCase,
};
use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use super::error_response::AppError;
use crate::AppState;

/// Health check endpoint
pub async fn health_handler() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"status": "healthy"})))
}

/// Register a new user
pub async fn register_handler(
    State(state): State<AppState>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let use_case = RegisterUserUseCase::new(
        state.user_repository,
        state.password_hasher.clone(),
        state.jwt_service.clone(),
    );

    let response = use_case.execute(request).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "user_id": response.user_id,
            "email": response.email,
            "token": response.token,
        })),
    ))
}

/// Login an existing user
pub async fn login_handler(
    State(state): State<AppState>,
    Json(request): Json<LoginUserRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let use_case = LoginUserUseCase::new(
        state.user_repository,
        state.password_hasher.clone(),
        state.jwt_service.clone(),
    );

    let response = use_case.execute(request).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "user_id": response.user_id,
            "email": response.email,
            "token": response.token,
        })),
    ))
}


