// crates/domain/src/posts/value_objects.rs

use crate::common::error::DomainError;
use serde::{Deserialize, Serialize};

// PostTitle: A validated title for a post
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostTitle(String);

impl PostTitle {
    pub fn new(title: impl Into<String>) -> Result<Self, DomainError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(DomainError::Validation("Post title cannot be empty".into()));
        }
        if title.len() > 200 {
            return Err(DomainError::Validation("Post title cannot exceed 200 characters".into()));
        }
        Ok(Self(title))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// PostContent: The body content of a post
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostContent(String);

impl PostContent {
    pub fn new(content: impl Into<String>) -> Result<Self, DomainError> {
        let content = content.into();
        if content.trim().is_empty() {
            return Err(DomainError::Validation("Post content cannot be empty".into()));
        }
        Ok(Self(content))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
