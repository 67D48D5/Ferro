// services/auth/src/adapters/mod.rs

mod jwt_adapter;
mod password_adapter;

pub use jwt_adapter::JwtAdapter;
pub use password_adapter::PasswordHasherAdapter;

