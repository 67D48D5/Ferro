// services/auth/src/graphql/schema.rs

use super::mutations::MutationRoot;
use super::queries::QueryRoot;
use async_graphql::{EmptySubscription, Schema};

pub type GraphQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema() -> GraphQLSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
