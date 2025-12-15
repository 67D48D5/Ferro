// crates/domain/src/comments/value_objects.rs

use crate::common::error::DomainError;
use serde::{Deserialize, Serialize};

// CommentContent: The body content of a comment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentContent(String);

impl CommentContent {
    pub fn new(content: impl Into<String>) -> Result<Self, DomainError> {
        let content = content.into();
        if content.trim().is_empty() {
            return Err(DomainError::Validation("Comment content cannot be empty".into()));
        }
        if content.len() > 2000 {
            return Err(DomainError::Validation("Comment content cannot exceed 2000 characters".into()));
        }
        Ok(Self(content))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
