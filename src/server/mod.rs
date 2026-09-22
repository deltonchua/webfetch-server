mod config;
mod markdown;
mod screenshot;
mod status;

use axum::{Router, http::StatusCode};
pub use config::Config;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

pub fn build(config: Config) -> Router {
    let routes = Router::new()
        .merge(status::route())
        .merge(markdown::route())
        .merge(screenshot::route());

    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_millis(config.timeout_ms as u64),
        ));

    Router::new()
        .nest("/v1", routes)
        .route_layer(layers)
        .with_state(config)
}
