// services/auth/src/handlers/comment_handlers.rs

use application::comments::{
    CreateCommentRequest, CreateCommentUseCase, ListCommentsUseCase,
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

/// Create a new comment on a post
pub async fn create_comment_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(post_id): Path<Uuid>,
    Json(request): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let use_case =
        CreateCommentUseCase::new(state.comment_repository, state.post_repository.clone());

    let response = use_case.execute(request, post_id, auth_user.user_id).await?;

    Ok((StatusCode::CREATED, Json(json!(response))))
}

/// List comments for a post with pagination
pub async fn list_comments_handler(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Value>, AppError> {
    let use_case = ListCommentsUseCase::new(state.comment_repository);

    let response = use_case
        .execute(post_id, params.limit, params.offset)
        .await?;

    Ok(Json(json!(response)))
}
