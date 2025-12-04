# Ferro Auth API Documentation

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

### 1. Health Check

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

### 2. Register User

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

### 3. Login User

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
