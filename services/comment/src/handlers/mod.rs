// services/comment/src/handlers/mod.rs

mod comment_handlers;
mod common;
mod error_response;

pub use comment_handlers::{create_comment_handler, health_handler, list_comments_handler};
pub use error_response::AppError;
