// services/auth/src/graphql/queries.rs

use async_graphql::{Object, Result};

use crate::AppState;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check
    async fn health(&self) -> &str {
        "healthy"
    }
}
