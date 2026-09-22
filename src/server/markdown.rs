use crate::{
    browser::{BrowserRequest, HtmlRequest},
    error::Error,
    server::Config,
};
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::oneshot;
use url::Url;

pub fn route() -> Router<Config> {
    Router::new().route("/markdown", get(markdown))
}

async fn markdown(State(config): State<Config>, Query(params): Query<Params>) -> impl IntoResponse {
    let _permit = match config.semaphore.try_acquire() {
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, "Server Busy").into_response(),
        Ok(p) => p,
    };

    // Validation
    if params.vw == 0 || params.vh == 0 || params.vw * params.vh > config.max_viewport {
        return (StatusCode::BAD_REQUEST, "Invalid Image Dimension").into_response();
    }
    if params.timeout_ms == 0 {
        return (StatusCode::BAD_REQUEST, "Invalid Timeout").into_response();
    }

    let result = async {
        // Schedule browser task
        let (tx, rx) = oneshot::channel();
        config
            .browser_tx
            .send(BrowserRequest::Html(HtmlRequest {
                url: params.url,
                width: params.vw,
                height: params.vh,
                timeout_ms: params.timeout_ms.min(config.timeout_ms),
                tx,
            }))
            .await
            .map_err(|_| Error::ChannelSend)?;

        // Convert html to markdown
        let html = rx.await.map_err(|_| Error::ChannelRecv)?;
        let markdown = config.md_converter.convert(&html)?;

        Ok::<_, Error>(markdown)
    }
    .await;
    match result {
        Ok(markdown) => Json(json!({"markdown": markdown})).into_response(),
        Err(err) => err.into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct Params {
    url: Url,

    #[serde(default = "default_vw")]
    vw: u32,

    #[serde(default = "default_vh")]
    vh: u32,

    #[serde(default = "default_timeout_ms")]
    timeout_ms: u32,
}

fn default_vw() -> u32 {
    1920
}

fn default_vh() -> u32 {
    1080
}

fn default_timeout_ms() -> u32 {
    10_000
}
