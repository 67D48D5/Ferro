// crates/application/src/posts/create_post.rs

use domain::common::error::DomainError;
use domain::posts::{Post, PostContent, PostRepository, PostTitle};
use uuid::Uuid;

use super::dtos::{CreatePostRequest, PostResponse};

pub struct CreatePostUseCase<R>
where
    R: PostRepository,
{
    post_repository: R,
}

impl<R> CreatePostUseCase<R>
where
    R: PostRepository,
{
    pub fn new(post_repository: R) -> Self {
        Self { post_repository }
    }

    pub async fn execute(
        &self,
        request: CreatePostRequest,
        author_id: Uuid,
    ) -> Result<PostResponse, DomainError> {
        // Validate title and content
        let title = PostTitle::new(request.title)?;
        let content = PostContent::new(request.content)?;

        // Create post
        let post = Post::new(title, content, author_id);

        // Save post
        self.post_repository.save(post.clone()).await?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct MockPostRepository {
        posts: Arc<Mutex<Vec<Post>>>,
    }

    impl MockPostRepository {
        fn new() -> Self {
            Self {
                posts: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl Clone for MockPostRepository {
        fn clone(&self) -> Self {
            Self {
                posts: Arc::clone(&self.posts),
            }
        }
    }

    #[async_trait]
    impl PostRepository for MockPostRepository {
        async fn save(&self, post: Post) -> Result<(), DomainError> {
            self.posts.lock().unwrap().push(post);
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
            Ok(self
                .posts
                .lock()
                .unwrap()
                .iter()
                .find(|p| p.id == id)
                .cloned())
        }

        async fn find_all(&self, _limit: i64, _offset: i64) -> Result<Vec<Post>, DomainError> {
            Ok(self.posts.lock().unwrap().clone())
        }

        async fn find_by_author(
            &self,
            author_id: Uuid,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<Post>, DomainError> {
            Ok(self
                .posts
                .lock()
                .unwrap()
                .iter()
                .filter(|p| p.author_id == author_id)
                .cloned()
                .collect())
        }
    }

    #[tokio::test]
    async fn test_create_post_success() {
        let use_case = CreatePostUseCase::new(MockPostRepository::new());
        let author_id = Uuid::new_v4();

        let request = CreatePostRequest {
            title: "Test Post".to_string(),
            content: "This is a test post content".to_string(),
        };

        let result = use_case.execute(request, author_id).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.title, "Test Post");
        assert_eq!(response.content, "This is a test post content");
        assert_eq!(response.author_id, author_id.to_string());
    }

    #[tokio::test]
    async fn test_create_post_empty_title() {
        let use_case = CreatePostUseCase::new(MockPostRepository::new());
        let author_id = Uuid::new_v4();

        let request = CreatePostRequest {
            title: "".to_string(),
            content: "This is a test post content".to_string(),
        };

        let result = use_case.execute(request, author_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }

    #[tokio::test]
    async fn test_create_post_empty_content() {
        let use_case = CreatePostUseCase::new(MockPostRepository::new());
        let author_id = Uuid::new_v4();

        let request = CreatePostRequest {
            title: "Test Post".to_string(),
            content: "".to_string(),
        };

        let result = use_case.execute(request, author_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }
}
