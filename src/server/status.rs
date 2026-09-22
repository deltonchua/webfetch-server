use crate::server::Config;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub fn route() -> Router<Config> {
    Router::new().route("/status", get(status))
}

async fn status() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
