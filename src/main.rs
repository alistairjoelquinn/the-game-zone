mod aws;
mod database;
mod handlers;
mod middleware;
mod model;
mod state;
mod utils;

use anyhow::{Context, Result};
use axum::{routing::get, Extension, Router};
use middleware::log::LoggingLayer;
use state::State;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = utils::initialise_cors();
    let db = database::initialise_database().await?;
    let s3 = aws::s3::init_s3()
        .await
        .context("Failed to initialize S3 client")?;
    let state = Arc::new(State { db, s3 });

    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/user", get(handlers::get_user).post(handlers::post_user))
        .route("/koen", get(aws::s3::get_s3_object))
        .route(
            "/user/:id",
            get(handlers::get_user_by_id)
                .patch(handlers::patch_user)
                .delete(handlers::delete_user),
        )
        .route("/users", get(handlers::get_users))
        .nest_service("/static", ServeDir::new("static"))
        .layer(LoggingLayer)
        .layer(cors)
        .layer(Extension(state));

    let listener = TcpListener::bind("127.0.0.1:3333")
        .await
        .context("Failed to bind to port 3333")?;
    println!("Starting server on port 3333");

    axum::serve(listener, app)
        .await
        .context("Error starting server")?;

    Ok(())
}
