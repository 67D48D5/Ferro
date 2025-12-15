// services/auth/src/handlers/mod.rs

mod auth_handlers;
mod comment_handlers;
mod common;
mod error_response;
mod post_handlers;

pub use auth_handlers::{health_handler, login_handler, register_handler};
pub use comment_handlers::{create_comment_handler, list_comments_handler};
pub use post_handlers::{create_post_handler, get_post_handler, list_posts_handler};
