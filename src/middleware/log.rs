use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn log_incoming_request(request: Request, next: Next) -> Response {
    info!("Incoming request: {} {}", request.method(), request.uri(),);
    let response = next.run(request).await;
    response
}
