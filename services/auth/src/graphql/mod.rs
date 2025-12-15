// services/auth/src/graphql/mod.rs

mod handler;
mod mutations;
mod queries;
mod schema;
mod types;

pub use handler::{graphql_handler, graphql_playground};
pub use schema::build_schema;
