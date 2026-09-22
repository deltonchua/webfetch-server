use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("serde json error: {0}")]
    SerdeJson(#[from] serde_json::error::Error),

    #[error("channel receive error")]
    ChannelRecv,

    #[error("channel send error")]
    ChannelSend,

    #[error("task join error: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("timeout error: {0}")]
    Timeout(#[from] tokio::time::error::Elapsed),

    #[error("chromium cdp error: {0}")]
    ChromiumCdp(#[from] chromiumoxide::error::CdpError),

    #[error("browser error: {0}")]
    Browser(String),

    #[error("markdown error: {0}")]
    Markdown(String),

    #[error("libvips error: {0}")]
    Libvips(#[from] libvips::error::Error),

    #[error("image error: {0}")]
    Image(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!(%self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error. Try again later.",
        )
            .into_response()
    }
}
