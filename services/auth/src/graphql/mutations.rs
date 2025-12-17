// services/auth/src/graphql/mutations.rs

use super::types::AuthResponse;
use application::users::{
    LoginUserRequest, LoginUserUseCase, RegisterUserRequest, RegisterUserUseCase,
};
use async_graphql::{Context, Object, Result};

use crate::AppState;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Register a new user
    async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> Result<AuthResponse> {
        let state = ctx.data::<AppState>()?;

        let request = RegisterUserRequest { email, password };

        let use_case = RegisterUserUseCase::new(
            state.user_repository.clone(),
            state.password_hasher.clone(),
            state.jwt_service.clone(),
        );

        let response = use_case
            .execute(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Registration failed: {}", e)))?;

        Ok(AuthResponse {
            user_id: response.user_id,
            email: response.email,
            token: response.token,
        })
    }

    /// Login an existing user
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<AuthResponse> {
        let state = ctx.data::<AppState>()?;

        let request = LoginUserRequest { email, password };

        let use_case = LoginUserUseCase::new(
            state.user_repository.clone(),
            state.password_hasher.clone(),
            state.jwt_service.clone(),
        );

        let response = use_case
            .execute(request)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Login failed: {}", e)))?;

        Ok(AuthResponse {
            user_id: response.user_id,
            email: response.email,
            token: response.token,
        })
    }
}
