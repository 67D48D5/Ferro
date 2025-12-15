// services/auth/src/middleware/auth_middleware.rs

use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::AppState;

/// Extension type to carry authenticated user information
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

/// Middleware to verify JWT token and extract user information
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract the Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check for Bearer token format
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify the token
    let claims = state
        .jwt_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Parse user_id from claims
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Insert the authenticated user into request extensions
    let auth_user = AuthUser {
        user_id,
        email: claims.email,
    };
    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}
