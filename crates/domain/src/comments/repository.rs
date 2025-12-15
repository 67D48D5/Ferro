// crates/domain/src/comments/repository.rs

use super::aggregate::Comment;
use crate::common::error::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

// Repository interface for comments
#[async_trait]
pub trait CommentRepository: Send + Sync + Clone {
    async fn save(&self, comment: Comment) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, DomainError>;
    async fn find_by_post(&self, post_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Comment>, DomainError>;
}
