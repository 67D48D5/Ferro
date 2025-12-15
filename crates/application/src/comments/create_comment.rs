// crates/application/src/comments/create_comment.rs

use domain::comments::{Comment, CommentContent, CommentRepository};
use domain::common::error::DomainError;
use domain::posts::PostRepository;
use uuid::Uuid;

use super::dtos::{CommentResponse, CreateCommentRequest};

pub struct CreateCommentUseCase<R, P>
where
    R: CommentRepository,
    P: PostRepository,
{
    comment_repository: R,
    post_repository: P,
}

impl<R, P> CreateCommentUseCase<R, P>
where
    R: CommentRepository,
    P: PostRepository,
{
    pub fn new(comment_repository: R, post_repository: P) -> Self {
        Self {
            comment_repository,
            post_repository,
        }
    }

    pub async fn execute(
        &self,
        request: CreateCommentRequest,
        post_id: Uuid,
        author_id: Uuid,
    ) -> Result<CommentResponse, DomainError> {
        // Verify that the post exists
        self.post_repository
            .find_by_id(post_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Post not found".into()))?;

        // Validate content
        let content = CommentContent::new(request.content)?;

        // Create comment
        let comment = Comment::new(content, post_id, author_id);

        // Save comment
        self.comment_repository.save(comment.clone()).await?;

        Ok(CommentResponse {
            id: comment.id.to_string(),
            content: comment.content.as_str().to_string(),
            post_id: comment.post_id.to_string(),
            author_id: comment.author_id.to_string(),
            created_at: comment.created_at.to_rfc3339(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use domain::posts::{Post, PostContent, PostTitle};
    use std::sync::{Arc, Mutex};

    struct MockCommentRepository {
        comments: Arc<Mutex<Vec<Comment>>>,
    }

    impl MockCommentRepository {
        fn new() -> Self {
            Self {
                comments: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    impl Clone for MockCommentRepository {
        fn clone(&self) -> Self {
            Self {
                comments: Arc::clone(&self.comments),
            }
        }
    }

    #[async_trait]
    impl CommentRepository for MockCommentRepository {
        async fn save(&self, comment: Comment) -> Result<(), DomainError> {
            self.comments.lock().unwrap().push(comment);
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, DomainError> {
            Ok(self
                .comments
                .lock()
                .unwrap()
                .iter()
                .find(|c| c.id == id)
                .cloned())
        }

        async fn find_by_post(
            &self,
            post_id: Uuid,
            _limit: i64,
            _offset: i64,
        ) -> Result<Vec<Comment>, DomainError> {
            Ok(self
                .comments
                .lock()
                .unwrap()
                .iter()
                .filter(|c| c.post_id == post_id)
                .cloned()
                .collect())
        }
    }

    struct MockPostRepository {
        posts: Arc<Mutex<Vec<Post>>>,
    }

    impl MockPostRepository {
        fn new_with_post(post: Post) -> Self {
            Self {
                posts: Arc::new(Mutex::new(vec![post])),
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
    async fn test_create_comment_success() {
        let author_id = Uuid::new_v4();
        let title = PostTitle::new("Test Post").unwrap();
        let content = PostContent::new("Test content").unwrap();
        let post = Post::new(title, content, author_id);
        let post_id = post.id;

        let use_case = CreateCommentUseCase::new(
            MockCommentRepository::new(),
            MockPostRepository::new_with_post(post),
        );

        let request = CreateCommentRequest {
            content: "This is a test comment".to_string(),
        };

        let result = use_case.execute(request, post_id, author_id).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.content, "This is a test comment");
        assert_eq!(response.post_id, post_id.to_string());
        assert_eq!(response.author_id, author_id.to_string());
    }

    #[tokio::test]
    async fn test_create_comment_empty_content() {
        let author_id = Uuid::new_v4();
        let title = PostTitle::new("Test Post").unwrap();
        let content = PostContent::new("Test content").unwrap();
        let post = Post::new(title, content, author_id);
        let post_id = post.id;

        let use_case = CreateCommentUseCase::new(
            MockCommentRepository::new(),
            MockPostRepository::new_with_post(post),
        );

        let request = CreateCommentRequest {
            content: "".to_string(),
        };

        let result = use_case.execute(request, post_id, author_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }
}
