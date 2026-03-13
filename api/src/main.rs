mod routes;
mod types;

use axum::{Router, routing::post};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/v1/decide", post(routes::decide_route))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("PolicyGate API listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
