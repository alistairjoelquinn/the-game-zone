use axum::body::Body as BoxBody;
use axum::http::{Request, Response};
use std::convert::Infallible;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingService { inner: service }
    }
}

#[derive(Clone)]
pub struct LoggingService<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for LoggingService<S>
where
    S: Service<
            Request<ReqBody>,
            Response = Response<BoxBody>,
            Error = Infallible,
        > + Clone
        + Send
        + 'static,
    S::Future: Send,
    ReqBody: Send + 'static + std::fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        println!(
            "
            {} --------- Received request: {} {}",
            crate::utils::get_time().format("%Y-%m-%d %H:%M:%S"),
            req.method(),
            req.uri(),
        );
        self.inner.call(req)
    }
}
