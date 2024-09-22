use axum::{extract::Request, middleware::Next, response::Response};

pub async fn log_incoming_request(request: Request, next: Next) -> Response {
    println!(
        "
            {} --------- Received request: {} {}",
        crate::utils::get_time().format("%Y-%m-%d %H:%M:%S"),
        request.method(),
        request.uri(),
    );
    println!("log middlware 1");

    let response = next.run(request).await;
    println!("log middlware 2");
    response
}
