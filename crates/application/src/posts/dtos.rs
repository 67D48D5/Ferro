// crates/application/src/posts/dtos.rs

use serde::{Deserialize, Serialize};

// Create Post DTOs
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: String,
    pub updated_at: String,
}

// List Posts Response
#[derive(Debug, Serialize)]
pub struct ListPostsResponse {
    pub posts: Vec<PostResponse>,
    pub total: usize,
}
