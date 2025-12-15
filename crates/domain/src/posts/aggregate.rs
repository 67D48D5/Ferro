// crates/domain/src/posts/aggregate.rs

use super::value_objects::{PostContent, PostTitle};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: PostTitle,
    pub content: PostContent,
    pub author_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Post {
    // Factory method: Create a new post
    pub fn new(title: PostTitle, content: PostContent, author_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            author_id,
            created_at: now,
            updated_at: now,
        }
    }
}
