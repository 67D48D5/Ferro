// services/auth/src/adapters/password_adapter.rs

use application::users::PasswordVerifier;
use async_trait::async_trait;
use domain::common::error::DomainError;
use domain::users::{PasswordHash, PasswordHasher, PlainPassword};
use infrastructure::security::Argon2PasswordHasher;
use std::sync::Arc;

#[derive(Clone)]
pub struct PasswordHasherAdapter {
    hasher: Arc<Argon2PasswordHasher>,
}

impl PasswordHasherAdapter {
    pub fn new(hasher: Arc<Argon2PasswordHasher>) -> Self {
        Self { hasher }
    }
}

impl PasswordVerifier for PasswordHasherAdapter {
    fn verify(&self, plain_password: &str, password_hash: &str) -> Result<bool, DomainError> {
        self.hasher.verify(plain_password, password_hash)
    }
}

#[async_trait]
impl PasswordHasher for PasswordHasherAdapter {
    async fn hash(&self, password: PlainPassword) -> Result<PasswordHash, DomainError> {
        self.hasher.hash(password).await
    }
}




