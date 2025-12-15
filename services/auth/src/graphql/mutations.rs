// services/auth/src/graphql/mutations.rs

use super::types::{AuthResponse, Comment, Post};
use application::comments::{CreateCommentRequest, CreateCommentUseCase};
use application::posts::{CreatePostRequest, CreatePostUseCase};
use application::users::{
    LoginUserRequest, LoginUserUseCase, RegisterUserRequest, RegisterUserUseCase,
};
use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::AppState;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Register a new user
    async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> Result<AuthResponse> {
        let state = ctx.data::<AppState>()?;

        let request = RegisterUserRequest { email, password };

        let use_case = RegisterUserUseCase::new(
            state.user_repository.clone(),
            state.password_hasher.clone(),
            state.jwt_service.clone(),
        );

        let response = use_case
            .execute(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Registration failed: {}", e)))?;

        Ok(AuthResponse {
            user_id: response.user_id,
            email: response.email,
            token: response.token,
        })
    }

    /// Login an existing user
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<AuthResponse> {
        let state = ctx.data::<AppState>()?;

        let request = LoginUserRequest { email, password };

        let use_case = LoginUserUseCase::new(
            state.user_repository.clone(),
            state.password_hasher.clone(),
            state.jwt_service.clone(),
        );

        let response = use_case
            .execute(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Login failed: {}", e)))?;

        Ok(AuthResponse {
            user_id: response.user_id,
            email: response.email,
            token: response.token,
        })
    }

    /// Create a new post (requires authentication)
    async fn create_post(&self, ctx: &Context<'_>, title: String, content: String) -> Result<Post> {
        let state = ctx.data::<AppState>()?;
        
        // Get user_id from context (passed from auth header)
        let user_id = ctx
            .data_opt::<Uuid>()
            .ok_or_else(|| async_graphql::Error::new("Authentication required"))?;

        let request = CreatePostRequest { title, content };

        let use_case = CreatePostUseCase::new(state.post_repository.clone());

        let response = use_case
            .execute(request, *user_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create post: {}", e)))?;

        Ok(Post {
            id: response.id,
            title: response.title,
            content: response.content,
            author_id: response.author_id,
            created_at: response.created_at,
            updated_at: response.updated_at,
            _comments: (),
        })
    }

    /// Create a new comment on a post (requires authentication)
    async fn create_comment(
        &self,
        ctx: &Context<'_>,
        post_id: String,
        content: String,
    ) -> Result<Comment> {
        let state = ctx.data::<AppState>()?;
        
        // Get user_id from context (passed from auth header)
        let user_id = ctx
            .data_opt::<Uuid>()
            .ok_or_else(|| async_graphql::Error::new("Authentication required"))?;

        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|_| async_graphql::Error::new("Invalid post ID format"))?;

        let request = CreateCommentRequest { content };

        let use_case =
            CreateCommentUseCase::new(state.comment_repository.clone(), state.post_repository.clone());

        let response = use_case
            .execute(request, post_uuid, *user_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create comment: {}", e)))?;

        Ok(Comment {
            id: response.id,
            content: response.content,
            post_id: response.post_id,
            author_id: response.author_id,
            created_at: response.created_at,
        })
    }
}
