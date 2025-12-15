# Ferro API Documentation

## Base URL
```
http://localhost:8080
```

## Authentication
All endpoints that require authentication should include the JWT token in the Authorization header:
```
Authorization: Bearer <token>
```

---

## Endpoints

### Authentication Endpoints

#### 1. Health Check

Check if the service is running and healthy.

**Endpoint:** `GET /health`

**Response:**
- Status: `200 OK`
- Body:
```json
{
  "status": "healthy"
}
```

**Example:**
```bash
curl http://localhost:8080/health
```

---

#### 2. Register User

Register a new user account.

**Endpoint:** `POST /api/auth/register`

**Request Body:**
```json
{
  "email": "string",     // Valid email address
  "password": "string"   // Minimum 8 characters
}
```

**Response:**
- Status: `201 Created`
- Body:
```json
{
  "user_id": "uuid",
  "email": "string",
  "token": "jwt_token"
}
```

**Error Responses:**

- `400 Bad Request` - Invalid email format or password too short
```json
{
  "error": "Invalid email format"
}
```

- `409 Conflict` - User already exists
```json
{
  "error": "User with this email already exists"
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepass123"
  }'
```

---

#### 3. Login User

Authenticate an existing user.

**Endpoint:** `POST /api/auth/login`

**Request Body:**
```json
{
  "email": "string",
  "password": "string"
}
```

**Response:**
- Status: `200 OK`
- Body:
```json
{
  "user_id": "uuid",
  "email": "string",
  "token": "jwt_token"
}
```

**Error Responses:**

- `400 Bad Request` - Invalid credentials
```json
{
  "error": "Invalid credentials"
}
```

- `404 Not Found` - User not found
```json
{
  "error": "User not found"
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepass123"
  }'
```

---

### Post Endpoints

#### 4. Create Post

Create a new post (requires authentication).

**Endpoint:** `POST /api/posts`

**Authentication:** Required

**Request Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "string",      // Max 200 characters
  "content": "string"     // Required, any length
}
```

**Response:**
- Status: `201 Created`
- Body:
```json
{
  "id": "uuid",
  "title": "string",
  "content": "string",
  "author_id": "uuid",
  "created_at": "ISO8601 timestamp",
  "updated_at": "ISO8601 timestamp"
}
```

**Error Responses:**

- `400 Bad Request` - Invalid input
```json
{
  "error": "Post title cannot be empty"
}
```

- `401 Unauthorized` - Missing or invalid token
```json
{
  "error": "Unauthorized"
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/api/posts \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Post",
    "content": "This is the content of my first post"
  }'
```

---

#### 5. Get Post

Get a single post by ID.

**Endpoint:** `GET /api/posts/:post_id`

**Response:**
- Status: `200 OK`
- Body:
```json
{
  "id": "uuid",
  "title": "string",
  "content": "string",
  "author_id": "uuid",
  "created_at": "ISO8601 timestamp",
  "updated_at": "ISO8601 timestamp"
}
```

**Error Responses:**

- `404 Not Found` - Post not found
```json
{
  "error": "Post not found"
}
```

**Example:**
```bash
curl http://localhost:8080/api/posts/550e8400-e29b-41d4-a716-446655440000
```

---

#### 6. List Posts

List all posts with pagination.

**Endpoint:** `GET /api/posts?limit=20&offset=0`

**Query Parameters:**
- `limit` (optional): Number of posts to return (default: 20)
- `offset` (optional): Number of posts to skip (default: 0)

**Response:**
- Status: `200 OK`
- Body:
```json
{
  "posts": [
    {
      "id": "uuid",
      "title": "string",
      "content": "string",
      "author_id": "uuid",
      "created_at": "ISO8601 timestamp",
      "updated_at": "ISO8601 timestamp"
    }
  ],
  "count": 10
}
```

**Note:** `count` represents the number of posts returned in the current page, not the total number of posts in the database.

**Example:**
```bash
curl "http://localhost:8080/api/posts?limit=10&offset=0"
```

---

### Comment Endpoints

#### 7. Create Comment

Create a new comment on a post (requires authentication).

**Endpoint:** `POST /api/posts/:post_id/comments`

**Authentication:** Required

**Request Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "content": "string"     // Required, max 2000 characters
}
```

**Response:**
- Status: `201 Created`
- Body:
```json
{
  "id": "uuid",
  "content": "string",
  "post_id": "uuid",
  "author_id": "uuid",
  "created_at": "ISO8601 timestamp"
}
```

**Error Responses:**

- `400 Bad Request` - Invalid input
```json
{
  "error": "Comment content cannot be empty"
}
```

- `401 Unauthorized` - Missing or invalid token
```json
{
  "error": "Unauthorized"
}
```

- `404 Not Found` - Post not found
```json
{
  "error": "Post not found"
}
```

**Example:**
```bash
curl -X POST http://localhost:8080/api/posts/550e8400-e29b-41d4-a716-446655440000/comments \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "Great post! Thanks for sharing."
  }'
```

---

#### 8. List Comments

List all comments for a post with pagination.

**Endpoint:** `GET /api/posts/:post_id/comments?limit=50&offset=0`

**Query Parameters:**
- `limit` (optional): Number of comments to return (default: 50)
- `offset` (optional): Number of comments to skip (default: 0)

**Response:**
- Status: `200 OK`
- Body:
```json
{
  "comments": [
    {
      "id": "uuid",
      "content": "string",
      "post_id": "uuid",
      "author_id": "uuid",
      "created_at": "ISO8601 timestamp"
    }
  ],
  "count": 5
}
```

**Note:** `count` represents the number of comments returned in the current page, not the total number of comments for the post.

**Example:**
```bash
curl "http://localhost:8080/api/posts/550e8400-e29b-41d4-a716-446655440000/comments?limit=20&offset=0"
```

---

## Error Codes

| Status Code | Description |
|-------------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request - Invalid input |
| 404 | Not Found - Resource doesn't exist |
| 409 | Conflict - Resource already exists |
| 500 | Internal Server Error |

---

## JWT Token

The JWT token returned from register and login endpoints contains:

**Claims:**
- `sub`: User ID (UUID)
- `email`: User email address
- `exp`: Expiration timestamp
- `iat`: Issued at timestamp

**Token Validity:**
- Default: 24 hours (configurable via `JWT_EXPIRATION_HOURS`)

**Usage:**
```bash
# Use the token in subsequent requests
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:8080/api/protected-endpoint
```

---

## Validation Rules

### Email
- Must contain '@' symbol
- Format: `local@domain`
- Example: `user@example.com`

### Password
- Minimum length: 8 characters
- No maximum length enforced
- Stored using Argon2 hashing

---

## Testing with Different Tools

### cURL
```bash
# Register
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"testpass123"}'

# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"testpass123"}'
```

### HTTPie
```bash
# Register
http POST localhost:8080/api/auth/register \
  email=test@example.com password=testpass123

# Login
http POST localhost:8080/api/auth/login \
  email=test@example.com password=testpass123
```

### Postman
1. Create a new request
2. Set method to POST
3. Set URL to `http://localhost:8080/api/auth/register`
4. Go to Body tab
5. Select "raw" and "JSON"
6. Enter the JSON body
7. Click Send

---

## Rate Limiting

Currently, no rate limiting is implemented at the application level. 

**Production Recommendation:** Implement rate limiting at the API gateway or reverse proxy level (nginx, Caddy, etc.)

---

## CORS

CORS is not currently configured. 

**Production Recommendation:** Configure CORS in the Axum router if serving a web frontend:

```rust
use tower_http::cors::CorsLayer;

let app = Router::new()
    .route("/api/auth/register", post(register_handler))
    .layer(CorsLayer::permissive()); // Configure appropriately
```
