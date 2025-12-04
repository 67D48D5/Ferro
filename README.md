# Ferro

A generic, high-performance backend server framework written in Rust.

## Features

- **Domain-Driven Design (DDD) Architecture**: Clean separation of concerns with Domain, Application, Infrastructure, and Service layers
- **Production-Ready Authentication**: Secure user authentication with Argon2 password hashing and JWT tokens
- **Type-Safe Database Operations**: PostgreSQL integration with SQLx
- **RESTful API**: Built with Axum web framework
- **Comprehensive Logging**: Structured logging with Tracing
- **Async/Await**: Fully async runtime with Tokio

## Architecture

```
ferro/
├── crates/
│   ├── domain/           # Core business logic and entities
│   ├── application/      # Use cases and application services
│   └── infrastructure/   # External services (database, security)
├── services/
│   └── auth/            # Authentication service (HTTP API)
└── migrations/          # Database migrations
```

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

Create a `.env` file in the `services/auth` directory:

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

### Build and Run

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run the auth service
cd services/auth
cargo run
```

The server will start on `http://localhost:8080`

## API Endpoints

### Health Check

```bash
GET /health
```

Response:
```json
{
  "status": "healthy"
}
```

### Register User

```bash
POST /api/auth/register
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

### Login User

```bash
POST /api/auth/login
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
- **Entities**: User aggregate
- **Value Objects**: Email, PasswordHash, PlainPassword
- **Repository Interfaces**: UserRepository, PasswordHasher
- **Domain Errors**: Validation, AlreadyExists, NotFound, InfraError

### Application Layer (`crates/application`)
- **Use Cases**: RegisterUserUseCase, LoginUserUseCase
- **DTOs**: Request and Response data transfer objects
- **Traits**: TokenGenerator, PasswordVerifier

### Infrastructure Layer (`crates/infrastructure`)
- **Persistence**: PostgreSQL implementation of UserRepository
- **Security**: Argon2 password hasher, JWT service

### Service Layer (`services/auth`)
- **HTTP API**: Axum web server with REST endpoints
- **Configuration**: Environment-based configuration
- **Error Handling**: HTTP error responses
- **Adapters**: Bridges between application and infrastructure

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

