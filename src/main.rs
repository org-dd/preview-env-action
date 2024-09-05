use std::net::Ipv4Addr;

use axum::{http::Method, response::IntoResponse, routing::post, Router};
use tokio::net::TcpListener;
use tower_http::cors::{AllowHeaders, Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    fn new() -> Self {
        Self {}
    }
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/github", post(github_handler))
}

async fn github_handler() -> impl IntoResponse {
    "gi hubs"
}

pub fn get_address() -> String {
    format!("{}:{}", Ipv4Addr::UNSPECIFIED.to_string(), 8000)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(AllowHeaders::any())
        .allow_origin(Any);

    let state = AppState::new();
    let api = routes()
        .with_state(state)
        .layer(trace_layer)
        .layer(cors_layer);
    let address = get_address();
    let tcp_listener = TcpListener::bind(&address).await.unwrap();

    axum::serve(tcp_listener, api).await.unwrap();
}
