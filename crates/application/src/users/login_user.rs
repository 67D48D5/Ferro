// crates/application/src/users/login_user.rs

use domain::common::error::DomainError;
use domain::users::{Email, UserRepository};

use super::dtos::{LoginUserRequest, LoginUserResponse};
use super::register_user::TokenGenerator;

pub struct LoginUserUseCase<R, V, J>
where
    R: UserRepository,
    V: PasswordVerifier,
    J: TokenGenerator,
{
    user_repository: R,
    password_verifier: V,
    token_generator: J,
}

/// Trait for verifying passwords
pub trait PasswordVerifier: Send + Sync {
    fn verify(&self, plain_password: &str, password_hash: &str) -> Result<bool, DomainError>;
}

impl<R, V, J> LoginUserUseCase<R, V, J>
where
    R: UserRepository,
    V: PasswordVerifier,
    J: TokenGenerator,
{
    pub fn new(user_repository: R, password_verifier: V, token_generator: J) -> Self {
        Self {
            user_repository,
            password_verifier,
            token_generator,
        }
    }

    pub async fn execute(
        &self,
        request: LoginUserRequest,
    ) -> Result<LoginUserResponse, DomainError> {
        // Validate email format
        let email = Email::new(request.email)?;

        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;

        // Verify password
        if !self
            .password_verifier
            .verify(&request.password, user.password_hash.as_str())?
        {
            return Err(DomainError::Validation("Invalid credentials".into()));
        }

        // Generate token
        let token = self.token_generator.generate(user.id, user.email.as_str())?;

        Ok(LoginUserResponse {
            user_id: user.id.to_string(),
            email: user.email.as_str().to_string(),
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use domain::users::{PasswordHash, User};
    use std::sync::{Arc, Mutex};

    struct MockUserRepository {
        users: Arc<Mutex<Vec<User>>>,
    }

    impl MockUserRepository {
        fn new_with_user(user: User) -> Self {
            Self {
                users: Arc::new(Mutex::new(vec![user])),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn save(&self, user: User) -> Result<(), DomainError> {
            self.users.lock().unwrap().push(user);
            Ok(())
        }

        async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
            Ok(self
                .users
                .lock()
                .unwrap()
                .iter()
                .find(|u| u.email == *email)
                .cloned())
        }
    }

    struct MockPasswordVerifier {
        should_verify: bool,
    }

    impl PasswordVerifier for MockPasswordVerifier {
        fn verify(&self, _plain_password: &str, _password_hash: &str) -> Result<bool, DomainError> {
            Ok(self.should_verify)
        }
    }

    struct MockTokenGenerator;

    impl TokenGenerator for MockTokenGenerator {
        fn generate(&self, user_id: uuid::Uuid, _email: &str) -> Result<String, DomainError> {
            Ok(format!("token_{}", user_id))
        }
    }

    #[tokio::test]
    async fn test_login_user_success() {
        let email = Email::new("test@example.com").unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let user = User::new(email.clone(), password_hash);

        let use_case = LoginUserUseCase::new(
            MockUserRepository::new_with_user(user.clone()),
            MockPasswordVerifier { should_verify: true },
            MockTokenGenerator,
        );

        let request = LoginUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.email, "test@example.com");
        assert!(response.token.starts_with("token_"));
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let email = Email::new("test@example.com").unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let user = User::new(email, password_hash);

        let use_case = LoginUserUseCase::new(
            MockUserRepository::new_with_user(user),
            MockPasswordVerifier { should_verify: true },
            MockTokenGenerator,
        );

        let request = LoginUserRequest {
            email: "notfound@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_login_user_wrong_password() {
        let email = Email::new("test@example.com").unwrap();
        let password_hash = PasswordHash::new("hashed_password".to_string());
        let user = User::new(email, password_hash);

        let use_case = LoginUserUseCase::new(
            MockUserRepository::new_with_user(user),
            MockPasswordVerifier { should_verify: false },
            MockTokenGenerator,
        );

        let request = LoginUserRequest {
            email: "test@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }
}
