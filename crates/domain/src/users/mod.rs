// crates/domain/src/users/mod.rs

mod aggregate;
mod errors;
mod repository;
mod value_objects;

// Re-exports
pub use aggregate::User;
pub use repository::UserRepository;
pub use value_objects::{Email, PasswordHash};
