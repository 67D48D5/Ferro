// crates/infrastructure/src/persistence/postgres_user_repository.rs

use async_trait::async_trait;
use domain::common::error::DomainError;
use domain::users::{Email, PasswordHash, User, UserRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: User) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(user.id)
        .bind(user.email.as_str())
        .bind(user.password_hash.as_str())
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return DomainError::AlreadyExists("User with this email already exists".into());
                }
            }
            DomainError::InfraError(format!("Database error: {}", e))
        })?;

        Ok(())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        let result: Option<(uuid::Uuid, String, String, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            r#"
            SELECT id, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InfraError(format!("Database error: {}", e)))?;

        match result {
            Some((id, email_str, password_hash_str, created_at)) => {
                let email = Email::new(email_str)
                    .map_err(|e| DomainError::InfraError(format!("Invalid email in DB: {}", e)))?;
                let password_hash = PasswordHash::new(password_hash_str);

                Ok(Some(User {
                    id,
                    email,
                    password_hash,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }
}


