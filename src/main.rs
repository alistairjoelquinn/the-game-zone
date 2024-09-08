mod aws;
mod database;
mod handlers;
mod middleware;
mod model;
mod state;
mod utils;

use aws::s3::S3Client;
use axum::Extension;
use axum::{routing::get, serve, Router};
use middleware::log::LoggingLayer;
use state::State;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let db = database::initialise_database().await;
    let state = Arc::new(State { db });
    let cors = utils::initialise_cors();

    let app = Router::new()
        .route("/", get(handlers::home))
        .route("/user", get(handlers::get_user).post(handlers::post_user))
        .route(
            "/koen",
            get(|| async {
                println!("running Koen code");
                let s3: S3Client = S3Client::new().await.unwrap();
                println!("S3 client: {:?}", s3);

                let client = s3.client;
                println!("Client: {:?}", client);
            }),
        )
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

    let listener = match TcpListener::bind("127.0.0.1:3333").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 3333: {}", e);
            std::process::exit(1);
        }
    };

    println!("Starting server on port 3333");

    serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Error starting server: {}", e);
        std::process::exit(1);
    });
}
