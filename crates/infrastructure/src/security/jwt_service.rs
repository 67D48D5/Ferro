// crates/infrastructure/src/security/jwt_service.rs

use chrono::{Duration, Utc};
use domain::common::error::DomainError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // User ID
    pub email: String,    // User email
    pub exp: i64,         // Expiration time
    pub iat: i64,         // Issued at
}

pub struct JwtService {
    secret: String,
    expiration_hours: i64,
}

impl JwtService {
    pub fn new(secret: String, expiration_hours: i64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: Uuid, email: &str) -> Result<String, DomainError> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.expiration_hours);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::InfraError(format!("Failed to generate token: {}", e)))
    }

    /// Verify and decode a JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, DomainError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| DomainError::InfraError(format!("Invalid token: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        let jwt_service = JwtService::new("test_secret_key".to_string(), 24);
        let user_id = Uuid::new_v4();
        let email = "test@example.com";

        let token = jwt_service.generate_token(user_id, email).unwrap();
        let claims = jwt_service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_invalid_token() {
        let jwt_service = JwtService::new("test_secret_key".to_string(), 24);
        let result = jwt_service.verify_token("invalid_token");

        assert!(result.is_err());
    }
}
