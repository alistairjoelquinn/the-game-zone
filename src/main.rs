mod aws;
mod database;
mod handlers;
mod middleware;
mod model;
mod utils;

use crate::handlers::api;
use crate::handlers::components;
use crate::utils::handle_timeout_error;
use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Extension, Router,
};
use middleware::log::log_incoming_request;
use model::State;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::timeout::TimeoutLayer;
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

    let db = database::initialise_database().await?;
    let s3 = aws::s3::init_s3().await?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let state = Arc::new(State { db, s3, jwt_secret });

    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/login", post(handlers::login))
        .route("/logout", get(handlers::logout))
        .route("/games", get(handlers::games))
        .route("/images", get(aws::s3::get_s3_object))
        .nest("/components", components::init())
        .nest("/api", api::init())
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(axum::middleware::from_fn(log_incoming_request))
                .layer(utils::initialise_cors())
                .layer(Extension(state)),
        );

    let listener = TcpListener::bind("127.0.0.1:3333").await?;

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("Error detected by axum::serve : {:?}", err);
    }

    Ok(())
}
