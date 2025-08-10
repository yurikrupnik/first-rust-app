use std::sync::Arc;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

mod auth;
mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod services;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    let state = Arc::new(AppState::new(config).await?);

    let app = Router::new()
        .route("/api/health", get(handlers::health::health_check))
        .nest("/api/auth", auth_routes())
        .nest("/api/users", user_routes())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(from_fn_with_state(
                    state.clone(),
                    crate::middleware::auth::auth_middleware,
                )),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server running on http://0.0.0.0:8080");
    
    axum::serve(listener, app).await?;
    Ok(())
}

fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/refresh", post(handlers::auth::refresh))
}

fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(handlers::users::get_users))
        .route("/{id}", get(handlers::users::get_user))
        .route("/", post(handlers::users::create_user))
}