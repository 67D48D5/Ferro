// crates/domain/src/comments/mod.rs

mod aggregate;
mod repository;
mod value_objects;

// Re-exports
pub use aggregate::Comment;
pub use repository::CommentRepository;
pub use value_objects::CommentContent;
