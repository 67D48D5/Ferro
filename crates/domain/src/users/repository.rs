// crates/domain/src/users/repository.rs

use super::aggregate::User;
use super::value_objects::{Email, PasswordHash, PlainPassword};
use crate::common::error::DomainError;
use async_trait::async_trait;

// Repository interface
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> Result<(), DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;
}

// Password hasing interface (important: domain should not know about Argon2)
#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: PlainPassword) -> Result<PasswordHash, DomainError>;
}
