// services/post/src/adapters/jwt_adapter.rs

use domain::common::error::DomainError;
use infrastructure::security::{Claims, JwtService};
use std::sync::Arc;

#[derive(Clone)]
pub struct JwtAdapter {
    jwt_service: Arc<JwtService>,
}

impl JwtAdapter {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, DomainError> {
        self.jwt_service.verify_token(token)
    }
}
