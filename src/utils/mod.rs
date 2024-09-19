use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use chrono::{DateTime, Local};
use tower_http::cors::CorsLayer;

pub fn get_time() -> DateTime<Local> {
    let now = Local::now();
    now
}

pub fn initialise_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3333".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
}
