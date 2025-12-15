// crates/application/src/comments/dtos.rs

use serde::{Deserialize, Serialize};

// Create Comment DTOs
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: String,
    pub content: String,
    pub post_id: String,
    pub author_id: String,
    pub created_at: String,
}

// List Comments Response
#[derive(Debug, Serialize)]
pub struct ListCommentsResponse {
    pub comments: Vec<CommentResponse>,
    pub count: usize,
}
