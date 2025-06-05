use axum::{Router, routing::get};
use ld_core::api::mount;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "lambda server!" }))
        .merge(mount());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
