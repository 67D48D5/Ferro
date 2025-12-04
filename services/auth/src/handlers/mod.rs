// services/auth/src/handlers/mod.rs

mod auth_handlers;
mod error_response;

pub use auth_handlers::{register_handler, login_handler, health_handler};
