// services/post/src/handlers/mod.rs

mod common;
mod error_response;
mod post_handlers;

pub use error_response::AppError;
pub use post_handlers::{create_post_handler, get_post_handler, health_handler, list_posts_handler};
