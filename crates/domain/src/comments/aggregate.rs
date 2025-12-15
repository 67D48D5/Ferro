// crates/domain/src/comments/aggregate.rs

use super::value_objects::CommentContent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub content: CommentContent,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Comment {
    // Factory method: Create a new comment
    pub fn new(content: CommentContent, post_id: Uuid, author_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            post_id,
            author_id,
            created_at: chrono::Utc::now(),
        }
    }
}
