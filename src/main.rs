use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Server starting up");

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/ws", get(websocket_handler))
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

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(msg) => {
                if let axum::extract::ws::Message::Text(text) = msg {
                    println!("Received message: {:?}", text);
                    if socket
                        .send(axum::extract::ws::Message::Text(text))
                        .await
                        .is_err()
                    {
                        println!("Client disconnected");
                        return;
                    }
                }
            }
            Err(err) => {
                println!("Webdsocket error: {:?}", err);
                return;
            }
        }
    }
    println!("Client disconnected");
}

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}
