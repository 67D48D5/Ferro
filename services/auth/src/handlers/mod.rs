// services/auth/src/handlers/mod.rs

mod auth_handlers;
mod error_response;

pub use auth_handlers::{health_handler, login_handler, register_handler};
pub use error_response::AppError;
