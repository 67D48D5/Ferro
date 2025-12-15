// services/auth/src/adapters/jwt_adapter.rs

use application::users::TokenGenerator;
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

impl TokenGenerator for JwtAdapter {
    fn generate(&self, user_id: uuid::Uuid, email: &str) -> Result<String, DomainError> {
        self.jwt_service.generate_token(user_id, email)
    }
}




