// crates/application/src/comments/list_comments.rs

use domain::comments::CommentRepository;
use domain::common::error::DomainError;
use uuid::Uuid;

use super::dtos::{CommentResponse, ListCommentsResponse};

pub struct ListCommentsUseCase<R>
where
    R: CommentRepository,
{
    comment_repository: R,
}

impl<R> ListCommentsUseCase<R>
where
    R: CommentRepository,
{
    pub fn new(comment_repository: R) -> Self {
        Self { comment_repository }
    }

    pub async fn execute(&self, post_id: Uuid, limit: i64, offset: i64) -> Result<ListCommentsResponse, DomainError> {
        let comments = self.comment_repository.find_by_post(post_id, limit, offset).await?;
        let total = comments.len();

        let comments = comments
            .into_iter()
            .map(|comment| CommentResponse {
                id: comment.id.to_string(),
                content: comment.content.as_str().to_string(),
                post_id: comment.post_id.to_string(),
                author_id: comment.author_id.to_string(),
                created_at: comment.created_at.to_rfc3339(),
            })
            .collect();

        Ok(ListCommentsResponse { comments, total })
    }
}
