// crates/application/src/users/register_user.rs

use domain::common::error::DomainError;
use domain::users::{Email, PasswordHasher, PlainPassword, User, UserRepository};

use super::dtos::{RegisterUserRequest, RegisterUserResponse};

pub struct RegisterUserUseCase<R, H, J>
where
    R: UserRepository,
    H: PasswordHasher,
    J: TokenGenerator,
{
    user_repository: R,
    password_hasher: H,
    token_generator: J,
}

/// Trait for generating authentication tokens
pub trait TokenGenerator: Send + Sync {
    fn generate(&self, user_id: uuid::Uuid, email: &str) -> Result<String, DomainError>;
}

impl<R, H, J> RegisterUserUseCase<R, H, J>
where
    R: UserRepository,
    H: PasswordHasher,
    J: TokenGenerator,
{
    pub fn new(user_repository: R, password_hasher: H, token_generator: J) -> Self {
        Self {
            user_repository,
            password_hasher,
            token_generator,
        }
    }

    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, DomainError> {
        // Validate email format
        let email = Email::new(request.email)?;

        // Check if user already exists
        if self
            .user_repository
            .find_by_email(&email)
            .await?
            .is_some()
        {
            return Err(DomainError::AlreadyExists(
                "User with this email already exists".into(),
            ));
        }

        // Validate and hash password
        let plain_password = PlainPassword::new(request.password)?;
        let password_hash = self.password_hasher.hash(plain_password).await?;

        // Create user
        let user = User::new(email.clone(), password_hash);

        // Save user
        self.user_repository.save(user.clone()).await?;

        // Generate token
        let token = self.token_generator.generate(user.id, user.email.as_str())?;

        Ok(RegisterUserResponse {
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
    use std::sync::{Arc, Mutex};

    struct MockUserRepository {
        users: Arc<Mutex<Vec<User>>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(Vec::new())),
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

    struct MockPasswordHasher;

    #[async_trait]
    impl PasswordHasher for MockPasswordHasher {
        async fn hash(
            &self,
            password: PlainPassword,
        ) -> Result<domain::users::PasswordHash, DomainError> {
            Ok(domain::users::PasswordHash::new(format!(
                "hashed_{}",
                password.as_str()
            )))
        }
    }

    struct MockTokenGenerator;

    impl TokenGenerator for MockTokenGenerator {
        fn generate(&self, user_id: uuid::Uuid, _email: &str) -> Result<String, DomainError> {
            Ok(format!("token_{}", user_id))
        }
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let use_case = RegisterUserUseCase::new(
            MockUserRepository::new(),
            MockPasswordHasher,
            MockTokenGenerator,
        );

        let request = RegisterUserRequest {
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
    async fn test_register_user_duplicate_email() {
        let repo = MockUserRepository::new();
        let use_case = RegisterUserUseCase::new(repo, MockPasswordHasher, MockTokenGenerator);

        let request = RegisterUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        // First registration should succeed
        use_case.execute(request.clone()).await.unwrap();

        // Second registration should fail
        let result = use_case.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::AlreadyExists(_)));
    }

    #[tokio::test]
    async fn test_register_user_invalid_email() {
        let use_case = RegisterUserUseCase::new(
            MockUserRepository::new(),
            MockPasswordHasher,
            MockTokenGenerator,
        );

        let request = RegisterUserRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }

    #[tokio::test]
    async fn test_register_user_short_password() {
        let use_case = RegisterUserUseCase::new(
            MockUserRepository::new(),
            MockPasswordHasher,
            MockTokenGenerator,
        );

        let request = RegisterUserRequest {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
        };

        let result = use_case.execute(request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::Validation(_)));
    }
}
