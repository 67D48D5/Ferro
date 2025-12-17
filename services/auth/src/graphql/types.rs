// services/auth/src/graphql/types.rs

use async_graphql::SimpleObject;

/// Response for register and login mutations
#[derive(SimpleObject, Clone)]
pub struct AuthResponse {
    #[graphql(name = "user_id")]
    pub user_id: String,
    pub email: String,
    pub token: String,
}
