# Ferro Auth Service Environment Variables

## Database Configuration
# PostgreSQL connection string
# Format: postgres://username:password@host:port/database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/ferro

## JWT Configuration
# Secret key for signing JWT tokens
# IMPORTANT: Use a strong, randomly generated secret in production!
# You can generate one with: openssl rand -hex 32
JWT_SECRET=change-this-secret-in-production

# JWT token expiration time in hours
JWT_EXPIRATION_HOURS=24

## Server Configuration
# Host address to bind the server
# Use 0.0.0.0 to listen on all interfaces
SERVER_HOST=0.0.0.0

# Port number for the HTTP server
SERVER_PORT=8080

## Logging Configuration
# Rust log level (trace, debug, info, warn, error)
# Example: RUST_LOG=auth=debug,tower_http=debug
# RUST_LOG=auth=info
