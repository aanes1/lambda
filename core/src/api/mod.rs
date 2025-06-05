use axum::{Router, routing::get};

pub fn mount() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}
