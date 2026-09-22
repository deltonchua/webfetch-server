use std::{env, path::PathBuf};
use tokio::{fs, sync::oneshot};
use tokio_util::sync::CancellationToken;
use url::Url;
use webfetch_server::{
    browser::{self, BrowserRequest, HtmlRequest, ScreenshotRequest},
    error::{Error, Result},
    image::ImageFormat,
};

fn env_path(name: &str) -> PathBuf {
    env::var(name)
        .unwrap_or_else(|_| panic!("missing env var: {name}"))
        .into()
}

#[tokio::test]
#[ignore]
async fn test_browser_html() -> Result<()> {
    let chromium_path = env_path("WEBFETCH_TEST_CHROMIUM");
    let user_data_dir = env_path("WEBFETCH_TEST_PROFILE");

    let (mtx, mrx) = tokio::sync::mpsc::channel(1);
    let token = CancellationToken::new();
    let browser_handle = browser::launch(
        chromium_path,
        user_data_dir,
        false,
        true,
        mrx,
        token.clone(),
    )
    .await?;

    let request_handle = tokio::spawn(async move {
        let (otx, orx) = oneshot::channel();
        mtx.send(BrowserRequest::Html(HtmlRequest {
            url: Url::parse("https://www.cnbc.com")?,
            width: 1080,
            height: 1080,
            timeout_ms: 10_000,
            tx: otx,
        }))
        .await
        .map_err(|_| Error::ChannelSend)?;

        let html = orx.await.map_err(|_| Error::ChannelRecv)?;
        let out = std::env::temp_dir().join("sample.html");
        fs::write(out, html).await?;

        Ok::<_, Error>(())
    });

    request_handle.await??;
    token.cancel();
    browser_handle.await?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_browser_screenshot() -> Result<()> {
    let chromium_path = env_path("WEBFETCH_TEST_CHROMIUM");
    let user_data_dir = env_path("WEBFETCH_TEST_PROFILE");

    let (mtx, mrx) = tokio::sync::mpsc::channel(1);
    let token = CancellationToken::new();
    let browser_handle = browser::launch(
        chromium_path,
        user_data_dir,
        false,
        true,
        mrx,
        token.clone(),
    )
    .await?;

    let request_handle = tokio::spawn(async move {
        let (otx, orx) = oneshot::channel();
        mtx.send(BrowserRequest::Screenshot(ScreenshotRequest {
            url: Url::parse("https://www.cnbc.com")?,
            width: 1080,
            height: 1080,
            frames: 10,
            format: ImageFormat::Webp,
            timeout_ms: 10_000,
            tx: otx,
        }))
        .await
        .map_err(|_| Error::ChannelSend)?;

        let image = orx.await.map_err(|_| Error::ChannelRecv)?;
        let out = std::env::temp_dir().join("sample.webp");
        fs::write(out, image).await?;

        Ok::<_, Error>(())
    });

    request_handle.await??;
    token.cancel();
    browser_handle.await?;
    Ok(())
}
