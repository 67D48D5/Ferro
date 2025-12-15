# Ferro GraphQL API Documentation

## Overview

Ferro now supports GraphQL alongside the existing REST API. The GraphQL endpoint provides a flexible and efficient way to query and manipulate data.

## GraphQL Endpoint

```
POST /graphql
```

## GraphQL Playground

For development and testing, a GraphQL Playground is available at:

```
GET /graphql/playground
```

## Authentication

For mutations that require authentication (createPost, createComment), include the JWT token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

## Schema

### Types

#### User
```graphql
type User {
  id: String!
  email: String!
  posts: [Post!]!
}
```

#### Post
```graphql
type Post {
  id: String!
  title: String!
  content: String!
  author_id: String!
  created_at: String!
  updated_at: String!
  comments: [Comment!]!
}
```

#### Comment
```graphql
type Comment {
  id: String!
  content: String!
  post_id: String!
  author_id: String!
  created_at: String!
}
```

#### AuthResponse
```graphql
type AuthResponse {
  user_id: String!
  email: String!
  token: String!
}
```

#### PostsResponse
```graphql
type PostsResponse {
  posts: [Post!]!
  count: Int!
}
```

#### CommentsResponse
```graphql
type CommentsResponse {
  comments: [Comment!]!
  count: Int!
}
```

## Queries

### health

Health check query.

```graphql
query {
  health
}
```

**Response:**
```json
{
  "data": {
    "health": "healthy"
  }
}
```

### post

Get a single post by ID.

**Arguments:**
- `id: String!` - The post ID (UUID format)

```graphql
query {
  post(id: "550e8400-e29b-41d4-a716-446655440000") {
    id
    title
    content
    author_id
    created_at
    updated_at
  }
}
```

### posts

List all posts with pagination.

**Arguments:**
- `limit: Int` - Number of posts to return (default: 20)
- `offset: Int` - Number of posts to skip (default: 0)

```graphql
query {
  posts(limit: 10, offset: 0) {
    posts {
      id
      title
      content
      author_id
      created_at
      updated_at
    }
    count
  }
}
```

### comments

List comments for a specific post with pagination.

**Arguments:**
- `post_id: String!` - The post ID (UUID format)
- `limit: Int` - Number of comments to return (default: 50)
- `offset: Int` - Number of comments to skip (default: 0)

```graphql
query {
  comments(post_id: "550e8400-e29b-41d4-a716-446655440000", limit: 20, offset: 0) {
    comments {
      id
      content
      author_id
      created_at
    }
    count
  }
}
```

## Mutations

### register

Register a new user account.

**Arguments:**
- `email: String!` - Valid email address
- `password: String!` - Password (minimum 8 characters)

```graphql
mutation {
  register(email: "user@example.com", password: "securepass123") {
    user_id
    email
    token
  }
}
```

**Response:**
```json
{
  "data": {
    "register": {
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "user@example.com",
      "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
    }
  }
}
```

### login

Authenticate an existing user.

**Arguments:**
- `email: String!` - User's email address
- `password: String!` - User's password

```graphql
mutation {
  login(email: "user@example.com", password: "securepass123") {
    user_id
    email
    token
  }
}
```

**Response:**
```json
{
  "data": {
    "login": {
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "user@example.com",
      "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
    }
  }
}
```

### createPost

Create a new post (requires authentication).

**Arguments:**
- `title: String!` - Post title
- `content: String!` - Post content

**Headers:**
```
Authorization: Bearer <your-jwt-token>
```

```graphql
mutation {
  createPost(title: "My First Post", content: "This is the content of my first post") {
    id
    title
    content
    author_id
    created_at
    updated_at
  }
}
```

**Response:**
```json
{
  "data": {
    "createPost": {
      "id": "post-uuid",
      "title": "My First Post",
      "content": "This is the content of my first post",
      "author_id": "user-uuid",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  }
}
```

### createComment

Create a new comment on a post (requires authentication).

**Arguments:**
- `post_id: String!` - The post ID (UUID format)
- `content: String!` - Comment content

**Headers:**
```
Authorization: Bearer <your-jwt-token>
```

```graphql
mutation {
  createComment(post_id: "post-uuid", content: "Great post! Thanks for sharing.") {
    id
    content
    post_id
    author_id
    created_at
  }
}
```

**Response:**
```json
{
  "data": {
    "createComment": {
      "id": "comment-uuid",
      "content": "Great post! Thanks for sharing.",
      "post_id": "post-uuid",
      "author_id": "user-uuid",
      "created_at": "2024-01-01T00:00:00Z"
    }
  }
}
```

## Example Usage with cURL

### Query Example

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ health }"
  }'
```

### Register User

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { register(email: \"user@example.com\", password: \"securepass123\") { user_id email token } }"
  }'
```

### Login User

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { login(email: \"user@example.com\", password: \"securepass123\") { user_id email token } }"
  }'
```

### List Posts

```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ posts(limit: 10) { posts { id title content author_id } count } }"
  }'
```

### Create Post (with Authentication)

```bash
# First, get the token from login/register
TOKEN="your-jwt-token-here"

curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "query": "mutation { createPost(title: \"My Post\", content: \"Post content\") { id title content author_id created_at } }"
  }'
```

### Create Comment (with Authentication)

```bash
TOKEN="your-jwt-token-here"
POST_ID="your-post-id-here"

curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d "{
    \"query\": \"mutation { createComment(post_id: \\\"$POST_ID\\\", content: \\\"Great post!\\\") { id content created_at } }\"
  }"
```

## Error Handling

GraphQL errors are returned in a standard format:

```json
{
  "errors": [
    {
      "message": "Authentication required",
      "path": ["createPost"]
    }
  ]
}
```

Common error messages:
- `"Authentication required"` - JWT token is missing or invalid for protected mutations
- `"Invalid post ID format"` - The provided post ID is not a valid UUID
- `"Failed to get post: ..."` - Post not found or database error
- `"Registration failed: ..."` - User already exists or validation error
- `"Login failed: ..."` - Invalid credentials

## Advantages of GraphQL

1. **Flexible Queries**: Request exactly the fields you need
2. **Single Endpoint**: All operations go through `/graphql`
3. **Strongly Typed**: Schema provides clear API contract
4. **Introspection**: Self-documenting API
5. **Efficient**: Reduce over-fetching and under-fetching
6. **Playground**: Interactive API explorer for development

## REST vs GraphQL

Both REST and GraphQL APIs are available in Ferro:

| Feature | REST | GraphQL |
|---------|------|---------|
| Endpoint | Multiple (`/api/posts`, `/api/auth/login`, etc.) | Single (`/graphql`) |
| Data Fetching | Fixed response structure | Flexible field selection |
| Multiple Resources | Multiple requests | Single request |
| Documentation | Manual docs | Schema + Playground |
| Caching | HTTP caching | Application-level |

Choose REST for:
- Simple CRUD operations
- Standard HTTP caching
- Wide client compatibility

Choose GraphQL for:
- Complex queries across multiple resources
- Mobile apps with limited bandwidth
- Flexible client requirements
- Rapid prototyping

## Next Steps

1. Visit http://localhost:8080/graphql/playground when the server is running
2. Explore the schema using the "Docs" tab in the playground
3. Try the example queries and mutations
4. Use the GraphQL API in your frontend application
