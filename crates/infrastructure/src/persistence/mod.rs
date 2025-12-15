// crates/infrastructure/src/persistence/mod.rs

mod postgres_comment_repository;
mod postgres_post_repository;
mod postgres_user_repository;

pub use postgres_comment_repository::PostgresCommentRepository;
pub use postgres_post_repository::PostgresPostRepository;
pub use postgres_user_repository::PostgresUserRepository;
