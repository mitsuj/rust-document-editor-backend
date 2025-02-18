use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Server starting up");

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}

async fn health() -> (StatusCode, Json<HealthCheckResponse>) {
    let response = HealthCheckResponse {
        status: "ok".to_string(),
    };
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}
