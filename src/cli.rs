pub use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};

/// Webfetch server
#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Server socket address
    #[arg(long, default_value = "127.0.0.1:3382")]
    pub addr: SocketAddr,

    /// Maximum number of concurrent requests
    #[arg(long, default_value_t = 20)]
    pub concurrency: usize,

    /// Request timeout in milliseconds
    #[arg(long, default_value_t = 20_000)]
    pub timeout_ms: u32,

    /// Path to the chromium executable
    #[arg(long)]
    pub chromium_path: PathBuf,

    /// Path to the chromium profile directory
    #[arg(long)]
    pub user_data_dir: PathBuf,

    /// Run the browser in headless mode
    #[arg(long)]
    pub headless: bool,

    /// Run the browser in incognito mode
    #[arg(long)]
    pub incognito: bool,

    /// Maximum viewport width times viewport height
    #[arg(long, default_value_t = 50_000_000)]
    pub max_viewport: u32,

    /// Directory for generated images
    #[arg(long, default_value = "/tmp/webfetch")]
    pub image_dir: PathBuf,

    /// Maximum number of vertical frames per page
    #[arg(long, default_value_t = 20)]
    pub max_frames: u32,
}
