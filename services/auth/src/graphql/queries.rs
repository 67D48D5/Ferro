// services/auth/src/graphql/queries.rs

use super::types::{Comment, CommentsResponse, Post, PostsResponse};
use application::comments::ListCommentsUseCase;
use application::posts::{GetPostUseCase, ListPostsUseCase};
use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::AppState;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check
    async fn health(&self) -> &str {
        "healthy"
    }

    /// Get a single post by ID
    async fn post(&self, ctx: &Context<'_>, id: String) -> Result<Post> {
        let state = ctx.data::<AppState>()?;
        let post_id = Uuid::parse_str(&id)
            .map_err(|_| async_graphql::Error::new("Invalid post ID format"))?;

        let use_case = GetPostUseCase::new(state.post_repository.clone());
        let response = use_case
            .execute(post_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get post: {}", e)))?;

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

    /// List all posts with pagination
    async fn posts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 20)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> Result<PostsResponse> {
        let state = ctx.data::<AppState>()?;

        let use_case = ListPostsUseCase::new(state.post_repository.clone());
        let response = use_case
            .execute(limit as i64, offset as i64)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to list posts: {}", e)))?;

        let posts = response
            .posts
            .into_iter()
            .map(|p| Post {
                id: p.id,
                title: p.title,
                content: p.content,
                author_id: p.author_id,
                created_at: p.created_at,
                updated_at: p.updated_at,
                _comments: (),
            })
            .collect();

        Ok(PostsResponse {
            posts,
            count: response.count as i32,
        })
    }

    /// List comments for a post with pagination
    async fn comments(
        &self,
        ctx: &Context<'_>,
        post_id: String,
        #[graphql(default = 50)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> Result<CommentsResponse> {
        let state = ctx.data::<AppState>()?;
        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|_| async_graphql::Error::new("Invalid post ID format"))?;

        let use_case = ListCommentsUseCase::new(state.comment_repository.clone());
        let response = use_case
            .execute(post_uuid, limit as i64, offset as i64)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to list comments: {}", e)))?;

        let comments = response
            .comments
            .into_iter()
            .map(|c| Comment {
                id: c.id,
                content: c.content,
                post_id: c.post_id,
                author_id: c.author_id,
                created_at: c.created_at,
            })
            .collect();

        Ok(CommentsResponse {
            comments,
            count: response.count as i32,
        })
    }
}
