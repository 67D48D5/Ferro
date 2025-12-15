// crates/infrastructure/src/persistence/postgres_post_repository.rs

use async_trait::async_trait;
use domain::common::error::DomainError;
use domain::posts::{Post, PostContent, PostRepository, PostTitle};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn save(&self, post: Post) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(post.id)
        .bind(post.title.as_str())
        .bind(post.content.as_str())
        .bind(post.author_id)
        .bind(post.created_at)
        .bind(post.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError> {
        let result: Option<(
            Uuid,
            String,
            String,
            Uuid,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
        )> = sqlx::query_as(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        match result {
            Some((id, title, content, author_id, created_at, updated_at)) => {
                let title = PostTitle::new(title)
                    .map_err(|e| DomainError::InfraError(format!("Invalid title in DB: {}", e)))?;
                let content = PostContent::new(content)
                    .map_err(|e| DomainError::InfraError(format!("Invalid content in DB: {}", e)))?;

                Ok(Some(Post {
                    id,
                    title,
                    content,
                    author_id,
                    created_at,
                    updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        let results: Vec<(
            Uuid,
            String,
            String,
            Uuid,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
        )> = sqlx::query_as(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        results
            .into_iter()
            .map(|(id, title, content, author_id, created_at, updated_at)| {
                let title = PostTitle::new(title)
                    .map_err(|e| DomainError::InfraError(format!("Invalid title in DB: {}", e)))?;
                let content = PostContent::new(content)
                    .map_err(|e| DomainError::InfraError(format!("Invalid content in DB: {}", e)))?;

                Ok(Post {
                    id,
                    title,
                    content,
                    author_id,
                    created_at,
                    updated_at,
                })
            })
            .collect()
    }

    async fn find_by_author(&self, author_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Post>, DomainError> {
        let results: Vec<(
            Uuid,
            String,
            String,
            Uuid,
            chrono::DateTime<chrono::Utc>,
            chrono::DateTime<chrono::Utc>,
        )> = sqlx::query_as(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            WHERE author_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(author_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        results
            .into_iter()
            .map(|(id, title, content, author_id, created_at, updated_at)| {
                let title = PostTitle::new(title)
                    .map_err(|e| DomainError::InfraError(format!("Invalid title in DB: {}", e)))?;
                let content = PostContent::new(content)
                    .map_err(|e| DomainError::InfraError(format!("Invalid content in DB: {}", e)))?;

                Ok(Post {
                    id,
                    title,
                    content,
                    author_id,
                    created_at,
                    updated_at,
                })
            })
            .collect()
    }
}
