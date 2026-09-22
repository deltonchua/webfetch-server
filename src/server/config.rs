use crate::browser::BrowserRequest;
use htmd::HtmlToMarkdown;
use std::{fmt, path::PathBuf, sync::Arc};
use tokio::sync::{Semaphore, mpsc::Sender};

#[derive(Clone)]
pub struct Config {
    pub concurrency: usize,
    pub timeout_ms: u32,
    pub md_converter: Arc<HtmlToMarkdown>,
    pub max_viewport: u32,
    pub image_dir: PathBuf,
    pub max_frames: u32,
    pub semaphore: Arc<Semaphore>,
    pub browser_tx: Sender<BrowserRequest>,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("concurrency", &self.concurrency)
            .field("timeout_ms", &self.timeout_ms)
            // .field("md_converter", &self.md_converter)
            .field("max_viewport", &self.max_viewport)
            .field("image_dir", &self.image_dir)
            .field("max_frames", &self.max_frames)
            .field("semaphore", &self.semaphore)
            .field("browser_tx", &self.browser_tx)
            .finish()
    }
}
