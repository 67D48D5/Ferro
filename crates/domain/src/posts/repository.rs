// crates/domain/src/posts/repository.rs

use super::aggregate::Post;
use crate::common::error::DomainError;
use async_trait::async_trait;
use uuid::Uuid;

// Repository interface for posts
#[async_trait]
pub trait PostRepository: Send + Sync + Clone {
    async fn save(&self, post: Post) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError>;
    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError>;
    async fn find_by_author(&self, author_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError>;
}
