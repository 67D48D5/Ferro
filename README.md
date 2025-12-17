# Ferro

A generic, high-performance backend server framework written in Rust.

## Features

- **Domain-Driven Design (DDD) Architecture**: Clean separation of concerns with Domain, Application, Infrastructure, and Service layers
- **Production-Ready Authentication**: Secure user authentication with Argon2 password hashing and JWT tokens
- **Posting and Comment System**: Full-featured blog/forum-style posting with threaded comments
- **Type-Safe Database Operations**: PostgreSQL integration with SQLx
- **RESTful API**: Built with Axum web framework
- **GraphQL API**: Full GraphQL support with async-graphql and interactive playground
- **JWT Authentication Middleware**: Secure protected endpoints
- **Comprehensive Logging**: Structured logging with Tracing
- **Async/Await**: Fully async runtime with Tokio

## Architecture

```
ferro/
├── crates/
│   ├── domain/           # Core business logic and entities (User, Post, Comment)
│   ├── application/      # Use cases and application services
│   └── infrastructure/   # External services (database, security)
├── services/
│   ├── auth/             # Authentication service (port 8080)
│   ├── post/             # Post management service (port 8081)
│   └── comment/          # Comment management service (port 8082)
└── migrations/           # Database migrations
```

## Microservices

Ferro implements a microservices architecture with three independent services:

1. **Auth Service** (port 8080): User registration, login, and JWT token generation
2. **Post Service** (port 8081): Post creation, retrieval, and listing
3. **Comment Service** (port 8082): Comment creation and listing on posts

Each service can be run independently and shares the same database.

## Getting Started

### Prerequisites

- Rust 1.75 or higher
- PostgreSQL 14 or higher
- Docker (optional, for running PostgreSQL)

### Running PostgreSQL with Docker

```bash
docker run --name ferro-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=ferro \
  -p 5432:5432 \
  -d postgres:16
```

### Environment Variables

Each service requires environment variables. Create a `.env` file in each service directory:

**Auth Service** (`services/auth/.env`):
```env
# Database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ferro

# JWT Configuration
JWT_SECRET=your-secret-key-change-this-in-production
JWT_EXPIRATION_HOURS=24

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

**Post Service** (`services/post/.env`):
```env
# Database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ferro

# JWT Configuration (for token verification)
JWT_SECRET=your-secret-key-change-this-in-production

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8081
```

**Comment Service** (`services/comment/.env`):
```env
# Database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ferro

# JWT Configuration (for token verification)
JWT_SECRET=your-secret-key-change-this-in-production

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8082
```

### Build and Run

```bash
# Build all services
cargo build

# Run tests
cargo test

# Run each service in separate terminals:

# Terminal 1: Auth service
cd services/auth
cargo run

# Terminal 2: Post service
cd services/post
cargo run

# Terminal 3: Comment service
cd services/comment
cargo run
```

The services will start on:
- Auth service: `http://localhost:8080`
- Post service: `http://localhost:8081`
- Comment service: `http://localhost:8082`

## API Protocols

Ferro supports both REST and GraphQL APIs.

### REST API

Each service exposes its own REST API endpoints:

- **Auth Service** (localhost:8080): Authentication endpoints
- **Post Service** (localhost:8081): Post management endpoints
- **Comment Service** (localhost:8082): Comment management endpoints

See [API_DOCUMENTATION.md](API_DOCUMENTATION.md) for complete REST API reference.

### GraphQL API

The Auth service also provides a GraphQL API for authentication:

- **Endpoint**: `POST http://localhost:8080/graphql`
- **Playground**: `GET http://localhost:8080/graphql/playground` (interactive API explorer)

See [GRAPHQL_API.md](GRAPHQL_API.md) for complete GraphQL API documentation with examples.

Quick GraphQL example:
```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ health }"}'
```

Visit http://localhost:8080/graphql/playground for an interactive GraphQL explorer.

## API Endpoints

### Authentication Service (Port 8080)

#### Health Check

```bash
GET http://localhost:8080/health
```

Response:
```json
{
  "status": "healthy"
}
```

#### Register User

```bash
POST http://localhost:8080/api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

Response (201 Created):
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

#### Login User

```bash
POST http://localhost:8080/api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

Response (200 OK):
```json
{
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Post Service (Port 8081)

#### Create Post (Requires Authentication)

```bash
POST http://localhost:8081/api/posts
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "My First Post",
  "content": "This is the content of my first post"
}
```

Response (201 Created):
```json
{
  "id": "post-uuid",
  "title": "My First Post",
  "content": "This is the content of my first post",
  "author_id": "user-uuid",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### Get Post

```bash
GET http://localhost:8081/api/posts/:post_id
```

#### List Posts

```bash
GET http://localhost:8081/api/posts?limit=20&offset=0
```

Response:
```json
{
  "posts": [...],
  "count": 10
}
```

### Comment Service (Port 8082)

#### Create Comment (Requires Authentication)

```bash
POST http://localhost:8082/api/posts/:post_id/comments
Authorization: Bearer <token>
Content-Type: application/json

{
  "content": "Great post! Thanks for sharing."
}
```

Response (201 Created):
```json
{
  "id": "comment-uuid",
  "content": "Great post! Thanks for sharing.",
  "post_id": "post-uuid",
  "author_id": "user-uuid",
  "created_at": "2024-01-01T00:00:00Z"
}
```

#### List Comments

```bash
GET http://localhost:8082/api/posts/:post_id/comments?limit=50&offset=0
```

Response:
```json
{
  "comments": [...],
  "count": 5
}
```

## API Examples

### Using cURL

Register a new user:
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "testpassword123"
  }'
```

Login:
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "testpassword123"
  }'
```

## Security Features

- **Argon2 Password Hashing**: Industry-standard password hashing algorithm
- **JWT Authentication**: Secure token-based authentication
- **Password Validation**: Minimum 8 characters required
- **Email Validation**: Basic email format validation
- **SQL Injection Protection**: Parameterized queries with SQLx
- **HTTPS Ready**: Deploy behind a reverse proxy (nginx, Caddy) for TLS

## Testing

Run all tests:
```bash
cargo test
```

Run tests for a specific crate:
```bash
cargo test -p domain
cargo test -p application
cargo test -p infrastructure
```

## Project Structure

### Domain Layer (`crates/domain`)
- **Entities**: User, Post, Comment aggregates
- **Value Objects**: Email, PasswordHash, PlainPassword
- **Repository Interfaces**: UserRepository, PostRepository, CommentRepository, PasswordHasher
- **Domain Errors**: Validation, AlreadyExists, NotFound, InfraError

### Application Layer (`crates/application`)
- **Use Cases**: 
  - User: RegisterUserUseCase, LoginUserUseCase
  - Post: CreatePostUseCase, GetPostUseCase, ListPostsUseCase
  - Comment: CreateCommentUseCase, ListCommentsUseCase
- **DTOs**: Request and Response data transfer objects
- **Traits**: TokenGenerator, PasswordVerifier

### Infrastructure Layer (`crates/infrastructure`)
- **Persistence**: PostgreSQL implementation of repositories (User, Post, Comment)
- **Security**: Argon2 password hasher, JWT service

### Service Layer (Microservices)

#### Auth Service (`services/auth`)
- **Port**: 8080
- **Purpose**: User authentication and JWT token management
- **Endpoints**: Register, Login, GraphQL
- **Authentication**: Generates JWT tokens

#### Post Service (`services/post`)
- **Port**: 8081
- **Purpose**: Post creation, retrieval, and listing
- **Endpoints**: Create post (protected), Get post, List posts
- **Authentication**: Validates JWT tokens for protected endpoints

#### Comment Service (`services/comment`)
- **Port**: 8082
- **Purpose**: Comment creation and listing
- **Endpoints**: Create comment (protected), List comments
- **Authentication**: Validates JWT tokens for protected endpoints

## Development

### Adding a New Feature

1. Define domain entities and value objects in `crates/domain`
2. Create use cases in `crates/application`
3. Implement infrastructure services in `crates/infrastructure`
4. Add HTTP endpoints in `services/auth`
5. Write tests at each layer

### Database Migrations

Migrations are stored in the `migrations/` directory and are automatically run on startup.

To create a new migration:
```bash
# Create migration file
touch migrations/002_add_your_feature.sql
```

## Production Deployment

### Recommendations

1. **Use HTTPS**: Deploy behind a reverse proxy with TLS
2. **Change JWT Secret**: Use a strong, randomly generated secret
3. **Environment Variables**: Use a secure secret management system
4. **Database**: Use a managed PostgreSQL service or set up replication
5. **Monitoring**: Add metrics and alerting (Prometheus, Grafana)
6. **Rate Limiting**: Implement rate limiting at the API gateway level
7. **CORS**: Configure CORS appropriately for your frontend

### Docker Deployment

Build the Docker image:
```bash
docker build -t ferro-auth -f services/auth/Dockerfile .
```

Run the container:
```bash
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host:5432/ferro \
  -e JWT_SECRET=your-production-secret \
  ferro-auth
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

See LICENSE file for details.

