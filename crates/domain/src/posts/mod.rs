// crates/domain/src/posts/mod.rs

mod aggregate;
mod repository;
mod value_objects;

// Re-exports
pub use aggregate::Post;
pub use repository::PostRepository;
pub use value_objects::{PostContent, PostTitle};
