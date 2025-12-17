// services/post/src/main.rs

mod adapters;
mod config;
mod handlers;
mod middleware;

use adapters::JwtAdapter;
use anyhow::Result;
use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use config::Config;
use handlers::{create_post_handler, get_post_handler, health_handler, list_posts_handler};
use infrastructure::{
    persistence::PostgresPostRepository,
    security::JwtService,
};
use middleware::auth_middleware;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    post_repository: PostgresPostRepository,
    jwt_service: JwtAdapter,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "post=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting post service with config: {:?}", config.server_address());

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
    let post_repository = PostgresPostRepository::new(pool.clone());
    let jwt_service = JwtAdapter::new(Arc::new(JwtService::new(
        config.jwt_secret.clone(),
        24, // Default expiration hours
    )));

    // Create app state
    let state = AppState {
        post_repository,
        jwt_service,
    };

    // Build router with protected routes
    let protected_routes = Router::new()
        .route("/api/posts", post(create_post_handler))
        .route_layer(axum_middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/posts", get(list_posts_handler))
        .route("/api/posts/{post_id}", get(get_post_handler))
        .merge(protected_routes)
        .with_state(state);

    // Start server
    let addr = config.server_address();
    tracing::info!("Post service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
