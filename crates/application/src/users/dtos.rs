// crates/application/src/users/dtos.rs

use serde::{Deserialize, Serialize};

// Register User DTOs
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub user_id: String,
    pub email: String,
    pub token: String,
}

// Login User DTOs
#[derive(Debug, Clone, Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginUserResponse {
    pub user_id: String,
    pub email: String,
    pub token: String,
}
