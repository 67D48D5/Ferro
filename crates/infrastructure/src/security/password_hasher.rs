// crates/infrastructure/src/security/password_hasher.rs

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher as Argon2Trait, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use async_trait::async_trait;
use domain::common::error::DomainError;
use domain::users::{PasswordHash as DomainPasswordHash, PasswordHasher, PlainPassword};

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }

    /// Verify a plain password against a hash
    pub fn verify(
        &self,
        plain_password: &str,
        password_hash: &str,
    ) -> Result<bool, DomainError> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| DomainError::InfraError(format!("Invalid password hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[async_trait]
impl PasswordHasher for Argon2PasswordHasher {
    async fn hash(&self, password: PlainPassword) -> Result<DomainPasswordHash, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(password.as_str().as_bytes(), &salt)
            .map_err(|e| DomainError::InfraError(format!("Failed to hash password: {}", e)))?;

        Ok(DomainPasswordHash::new(hash.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_hashing() {
        let hasher = Argon2PasswordHasher::new();
        let plain = PlainPassword::new("testpassword123").unwrap();

        let hash = hasher.hash(plain.clone()).await.unwrap();

        // Verify correct password
        assert!(hasher.verify("testpassword123", hash.as_str()).unwrap());

        // Verify incorrect password
        assert!(!hasher.verify("wrongpassword", hash.as_str()).unwrap());
    }
}
