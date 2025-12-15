// crates/application/src/posts/list_posts.rs

use domain::common::error::DomainError;
use domain::posts::PostRepository;

use super::dtos::{ListPostsResponse, PostResponse};

pub struct ListPostsUseCase<R>
where
    R: PostRepository,
{
    post_repository: R,
}

impl<R> ListPostsUseCase<R>
where
    R: PostRepository,
{
    pub fn new(post_repository: R) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, limit: i64, offset: i64) -> Result<ListPostsResponse, DomainError> {
        let posts = self.post_repository.find_all(limit, offset).await?;
        let count = posts.len();

        let posts = posts
            .into_iter()
            .map(|post| PostResponse {
                id: post.id.to_string(),
                title: post.title.as_str().to_string(),
                content: post.content.as_str().to_string(),
                author_id: post.author_id.to_string(),
                created_at: post.created_at.to_rfc3339(),
                updated_at: post.updated_at.to_rfc3339(),
            })
            .collect();

        Ok(ListPostsResponse { posts, count })
    }
}
