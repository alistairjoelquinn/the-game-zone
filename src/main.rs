mod aws;
mod database;
mod handlers;
mod middleware;
mod model;
mod state;
mod utils;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use middleware::log::log_incoming_request;
use state::State;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = utils::initialise_cors();
    let db = database::initialise_database().await?;
    let s3 = aws::s3::init_s3().await?;
    let state = Arc::new(State { db, s3 });

    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/login-field", get(handlers::login_field))
        .route("/login", post(handlers::login))
        .route("/image", get(aws::s3::get_s3_object))
        .route("/users", get(handlers::get_users))
        .route("/game-zone", get(handlers::game_zone))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(log_incoming_request))
                .layer(cors)
                .layer(Extension(state)),
        );

    let listener = TcpListener::bind("127.0.0.1:3333").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
