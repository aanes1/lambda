use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct Ctx {
    pub db: Pool<Postgres>,
}

pub fn mount(ctx: Ctx) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .with_state(ctx)
}
