// crates/application/src/posts/get_post.rs

use domain::common::error::DomainError;
use domain::posts::PostRepository;
use uuid::Uuid;

use super::dtos::PostResponse;

pub struct GetPostUseCase<R>
where
    R: PostRepository,
{
    post_repository: R,
}

impl<R> GetPostUseCase<R>
where
    R: PostRepository,
{
    pub fn new(post_repository: R) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, post_id: Uuid) -> Result<PostResponse, DomainError> {
        let post = self
            .post_repository
            .find_by_id(post_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Post not found".into()))?;

        Ok(PostResponse {
            id: post.id.to_string(),
            title: post.title.as_str().to_string(),
            content: post.content.as_str().to_string(),
            author_id: post.author_id.to_string(),
            created_at: post.created_at.to_rfc3339(),
            updated_at: post.updated_at.to_rfc3339(),
        })
    }
}
