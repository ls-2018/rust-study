mod auth;
mod request_id;
mod server_time;
use self::{request_id::set_request_id, server_time::ServerTimeLayer};
pub use super::User;
use axum::{Router, middleware::from_fn};
use http::Method;
use std::fmt;
use tower::ServiceBuilder;
use tower_http::cors::{self, CorsLayer};
use tower_http::{
    LatencyUnit,
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub trait TokenVerify {
    type Error: fmt::Debug;
    fn verify(&self, token: &str) -> Result<User, Self::Error>;
}

const REQUEST_ID_HEADER: &str = "x-request-id";
const SERVER_TIME_HEADER: &str = "x-server-time";

#[allow(unused)]
pub fn set_layer(app: Router) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO).latency_unit(LatencyUnit::Micros)),
            )
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
            .layer(from_fn(set_request_id))
            .layer(ServerTimeLayer),
    )
    .layer(
        CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::PUT])
            .allow_origin(cors::Any)
            .allow_headers(cors::Any),
    )
}
