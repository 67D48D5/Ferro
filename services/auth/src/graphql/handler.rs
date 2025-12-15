// services/auth/src/graphql/handler.rs

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap},
    response::{Html, IntoResponse},
    Extension,
};
use uuid::Uuid;

use crate::AppState;

use super::schema::GraphQLSchema;

/// GraphQL query handler
pub async fn graphql_handler(
    State(state): State<AppState>,
    schema: Extension<GraphQLSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();
    
    // Add app state to context
    request = request.data(state.clone());
    
    // Try to extract and verify JWT token if present
    if let Some(auth_header) = headers.get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // Verify the token and extract user_id
                if let Ok(claims) = state.jwt_service.verify_token(token) {
                    if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                        request = request.data(user_id);
                    }
                }
            }
        }
    }
    
    schema.execute(request).await.into()
}

/// GraphQL Playground handler (for development)
pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
