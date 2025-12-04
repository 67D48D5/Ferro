// crates/domain/src/users/value_objects.rs

use crate::common::error::DomainError;
use serde::{Deserialize, Serialize};

// Email: Must be in a valid format
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, DomainError> {
        let email = email.into();
        if !email.contains('@') {
            // @TODO: Add more robust email validation
            return Err(DomainError::Validation("Invalid email format".into()));
        }
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Password: A plain text password used for validation before hashing
#[derive(Debug, Clone)]
pub struct PlainPassword(String);

impl PlainPassword {
    pub fn new(password: impl Into<String>) -> Result<Self, DomainError> {
        let pwd = password.into();
        if pwd.len() < 8 {
            return Err(DomainError::Validation(
                "Password must be at least 8 chars".into(),
            ));
        }
        Ok(Self(pwd))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Hashed Password: A secure string to be stored in the DB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    // Assume the hash is already validated or add hash format validation logic
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
