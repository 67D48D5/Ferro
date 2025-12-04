// crates/domain/src/users/aggregate.rs

use super::value_objects::{Email, PasswordHash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub password_hash: PasswordHash, // 평문 비밀번호는 절대 들고 있지 않음
    pub created_at: chrono::DateTime<chrono::Utc>, // chrono 추가 필요
}

impl User {
    // 팩토리 메서드: 새 유저 생성
    pub fn new(email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            created_at: chrono::Utc::now(),
        }
    }
}
