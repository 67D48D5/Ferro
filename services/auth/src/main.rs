// services/auth/src/main.rs

mod adapters;
mod config;
mod handlers;

use adapters::{JwtAdapter, PasswordHasherAdapter};
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use config::Config;
use handlers::{health_handler, login_handler, register_handler};
use infrastructure::{
    persistence::PostgresUserRepository,
    security::{Argon2PasswordHasher, JwtService},
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    user_repository: PostgresUserRepository,
    password_hasher: PasswordHasherAdapter,
    jwt_service: JwtAdapter,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "auth=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting auth service with config: {:?}", config.server_address());

    // Connect to database
    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Connected to database");

    // Run migrations
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    tracing::info!("Migrations completed");

    // Create infrastructure components
    let user_repository = PostgresUserRepository::new(pool);
    let password_hasher = PasswordHasherAdapter::new(Arc::new(Argon2PasswordHasher::new()));
    let jwt_service = JwtAdapter::new(Arc::new(JwtService::new(
        config.jwt_secret.clone(),
        config.jwt_expiration_hours,
    )));

    // Create app state
    let state = AppState {
        user_repository,
        password_hasher,
        jwt_service,
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        .with_state(state);

    // Start server
    let addr = config.server_address();
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}





