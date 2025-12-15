// services/auth/src/graphql/types.rs

use async_graphql::{ComplexObject, Context, Result, SimpleObject};

/// GraphQL User type
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub email: String,
    #[graphql(skip)]
    pub _posts: (),
}

#[ComplexObject]
impl User {
    /// Get all posts by this user
    async fn posts(&self, _ctx: &Context<'_>) -> Result<Vec<Post>> {
        // This would typically fetch posts from repository
        // For now, return empty vector
        Ok(vec![])
    }
}

/// GraphQL Post type
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    #[graphql(name = "author_id")]
    pub author_id: String,
    #[graphql(name = "created_at")]
    pub created_at: String,
    #[graphql(name = "updated_at")]
    pub updated_at: String,
    #[graphql(skip)]
    pub _comments: (),
}

#[ComplexObject]
impl Post {
    /// Get all comments on this post
    async fn comments(&self, _ctx: &Context<'_>) -> Result<Vec<Comment>> {
        // This would typically fetch comments from repository
        // For now, return empty vector
        Ok(vec![])
    }
}

/// GraphQL Comment type
#[derive(SimpleObject, Clone)]
pub struct Comment {
    pub id: String,
    pub content: String,
    #[graphql(name = "post_id")]
    pub post_id: String,
    #[graphql(name = "author_id")]
    pub author_id: String,
    #[graphql(name = "created_at")]
    pub created_at: String,
}

/// Response for register and login mutations
#[derive(SimpleObject, Clone)]
pub struct AuthResponse {
    #[graphql(name = "user_id")]
    pub user_id: String,
    pub email: String,
    pub token: String,
}

/// Response for lists with pagination
#[derive(SimpleObject, Clone)]
pub struct PostsResponse {
    pub posts: Vec<Post>,
    pub count: i32,
}

#[derive(SimpleObject, Clone)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
    pub count: i32,
}
