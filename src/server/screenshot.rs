use crate::{
    browser::{BrowserRequest, ScreenshotRequest},
    error::Error,
    image::{self, ImageFormat},
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
use tokio::{fs, sync::oneshot, task::JoinSet};
use url::Url;

pub fn route() -> Router<Config> {
    Router::new().route("/screenshot", get(screenshot))
}

async fn screenshot(
    State(config): State<Config>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let _permit = match config.semaphore.try_acquire() {
        Err(_) => return (StatusCode::SERVICE_UNAVAILABLE, "Server Busy").into_response(),
        Ok(p) => p,
    };

    // Validation
    if params.vw == 0
        || params.vh == 0
        || params.ow == 0
        || params.frames == 0
        || params.ow > params.vw
        || params.vw * params.vh > config.max_viewport
    {
        return (StatusCode::BAD_REQUEST, "Invalid Image Dimension").into_response();
    }
    if params.timeout_ms == 0 {
        return (StatusCode::BAD_REQUEST, "Invalid Timeout").into_response();
    }

    let result = async {
        // Prepare output directory
        let out_dir = config.image_dir.join(format!(
            "{}{}",
            params.url.host_str().unwrap_or_default(),
            params.url.path()
        ));
        fs::create_dir_all(&out_dir).await?;

        // Schedule browser task
        let (tx, rx) = oneshot::channel();
        config
            .browser_tx
            .send(BrowserRequest::Screenshot(ScreenshotRequest {
                url: params.url,
                width: params.vw,
                height: params.vh,
                frames: params.frames.min(config.max_frames),
                format: params.format,
                timeout_ms: params.timeout_ms.min(config.timeout_ms),
                tx,
            }))
            .await
            .map_err(|_| Error::ChannelSend)?;

        // Resize and split screenshot vertically into frames
        let raw = rx.await.map_err(|_| Error::ChannelRecv)?;
        let resized = image::resize(&raw, params.ow, params.format)?;
        let oh = (params.ow as f64 / params.vw as f64) * params.vh as f64;
        let chunks = image::split_by_height(&resized, oh as u32, params.format)?;

        // Write each frame to disk
        let mut set = JoinSet::new();
        for (i, chunk) in chunks.enumerate() {
            let out = out_dir.join(format!("frame_{}{}", i, params.format.suffix()));
            set.spawn(async move {
                fs::write(&out, chunk?).await?;
                Ok::<_, Error>(out.to_string_lossy().into_owned())
            });
        }

        // Collect successful paths in ascending order
        let mut paths: Vec<String> = set
            .join_all()
            .await
            .into_iter()
            .filter_map(|res| res.ok())
            .collect();
        paths.sort();

        Ok::<_, Error>(paths)
    }
    .await;
    match result {
        Ok(paths) => Json(json!({"frames": paths})).into_response(),
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

    #[serde(default = "default_ow")]
    ow: u32,

    #[serde(default = "default_frames")]
    frames: u32,

    #[serde(default = "default_format")]
    format: ImageFormat,

    #[serde(default = "default_timeout_ms")]
    timeout_ms: u32,
}

fn default_vw() -> u32 {
    1920
}

fn default_vh() -> u32 {
    1080
}

fn default_ow() -> u32 {
    720
}

fn default_frames() -> u32 {
    5
}

fn default_format() -> ImageFormat {
    ImageFormat::Webp
}

fn default_timeout_ms() -> u32 {
    10_000
}
