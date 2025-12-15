// services/auth/src/handlers/post_handlers.rs

use application::posts::{
    CreatePostRequest, CreatePostUseCase, GetPostUseCase, ListPostsUseCase,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde_json::{json, Value};
use uuid::Uuid;

use super::common::PaginationParams;
use super::error_response::AppError;
use crate::{middleware::AuthUser, AppState};

/// Create a new post
pub async fn create_post_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let use_case = CreatePostUseCase::new(state.post_repository);

    let response = use_case.execute(request, auth_user.user_id).await?;

    Ok((StatusCode::CREATED, Json(json!(response))))
}

/// Get a single post by ID
pub async fn get_post_handler(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    let use_case = GetPostUseCase::new(state.post_repository);

    let response = use_case.execute(post_id).await?;

    Ok(Json(json!(response)))
}

/// List all posts with pagination
pub async fn list_posts_handler(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Value>, AppError> {
    let use_case = ListPostsUseCase::new(state.post_repository);

    let response = use_case.execute(params.limit, params.offset).await?;

    Ok(Json(json!(response)))
}
