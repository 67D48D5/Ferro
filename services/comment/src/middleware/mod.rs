// services/comment/src/middleware/mod.rs

mod auth_middleware;

pub use auth_middleware::{auth_middleware, AuthUser};
