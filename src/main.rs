mod database;
mod handlers;
mod middleware;
mod model;
mod state;
mod utils;

use axum::routing::get_service;
use axum::Extension;
use axum::{routing::get, serve, Router};
use middleware::log::LoggingLayer;
use state::State;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use utils::handle_internal_error;

#[tokio::main]
async fn main() {
    let db = database::initialise_database().await;
    let state = Arc::new(State { db });
    let cors = utils::initialise_cors();

    let app = Router::new()
        .nest_service(
            "/",
            get_service(ServeFile::new("static/index.html"))
                .handle_error(handle_internal_error),
        )
        .route("/user", get(handlers::get_user).post(handlers::post_user))
        .route(
            "/user/:id",
            get(handlers::get_user_by_id)
                .patch(handlers::patch_user)
                .delete(handlers::delete_user),
        )
        .route("/users", get(handlers::get_users))
        .route("/hello", get(handlers::hello))
        .nest_service("/static", ServeDir::new("static"))
        .layer(LoggingLayer)
        .layer(cors)
        .layer(Extension(state));

    let listener = match TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 3000: {}", e);
            std::process::exit(1);
        }
    };

    match serve(listener, app).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error starting server: {}", e);
            std::process::exit(1);
        }
    }
}
