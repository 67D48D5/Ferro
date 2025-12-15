// crates/infrastructure/src/persistence/postgres_comment_repository.rs

use async_trait::async_trait;
use domain::comments::{Comment, CommentContent, CommentRepository};
use domain::common::error::DomainError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresCommentRepository {
    pool: PgPool,
}

impl PostgresCommentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommentRepository for PostgresCommentRepository {
    async fn save(&self, comment: Comment) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO comments (id, content, post_id, author_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(comment.id)
        .bind(comment.content.as_str())
        .bind(comment.post_id)
        .bind(comment.author_id)
        .bind(comment.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Comment>, DomainError> {
        let result: Option<(
            Uuid,
            String,
            Uuid,
            Uuid,
            chrono::DateTime<chrono::Utc>,
        )> = sqlx::query_as(
            r#"
            SELECT id, content, post_id, author_id, created_at
            FROM comments
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        match result {
            Some((id, content, post_id, author_id, created_at)) => {
                let content = CommentContent::new(content)
                    .map_err(|e| DomainError::InfraError(format!("Invalid content in DB: {}", e)))?;

                Ok(Some(Comment {
                    id,
                    content,
                    post_id,
                    author_id,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_by_post(&self, post_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Comment>, DomainError> {
        let results: Vec<(
            Uuid,
            String,
            Uuid,
            Uuid,
            chrono::DateTime<chrono::Utc>,
        )> = sqlx::query_as(
            r#"
            SELECT id, content, post_id, author_id, created_at
            FROM comments
            WHERE post_id = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(post_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        results
            .into_iter()
            .map(|(id, content, post_id, author_id, created_at)| {
                let content = CommentContent::new(content)
                    .map_err(|e| DomainError::InfraError(format!("Invalid content in DB: {}", e)))?;

                Ok(Comment {
                    id,
                    content,
                    post_id,
                    author_id,
                    created_at,
                })
            })
            .collect()
    }
}
