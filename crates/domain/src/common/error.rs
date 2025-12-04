// crates/domain/src/common/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    // Business Rule Violation
    // Occurs when validation of input data fails, such as invalid email format or insufficient password length.
    // Good to map to 400 Bad Request.
    #[error("Validation failed: {0}")]
    Validation(String),

    // Data Conflict
    // Occurs when trying to create a resource that already exists, such as a duplicate email during registration.
    // Good to map to 409 Conflict.
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    // Data Not Found
    // Occurs when data is not found during a query operation.
    // Good to map to 404 Not Found.
    #[error("Resource not found: {0}")]
    NotFound(String),

    // Infrastructure Error
    // Occurs due to issues outside domain logic, such as DB connection failure or hashing library errors.
    // Protects the domain from depending on infrastructure technology by wrapping specific error types (e.g., sqlx::Error) in a String.
    // Good to map to 500 Internal Server Error.
    #[error("Infrastructure error: {0}")]
    InfraError(String),
}
