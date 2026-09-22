use std::sync::Arc;
use tokio::{
    net::TcpListener,
    signal,
    sync::{Semaphore, mpsc},
};
use tokio_util::sync::CancellationToken;
use webfetch_server::{
    browser::{self, BrowserRequest},
    cli::{Args, Parser},
    image, logging, markdown,
    server::{self, Config},
};

#[tokio::main]
async fn main() {
    // Enable logging
    logging::enable();

    // Parse cli arguments
    let args = Args::parse();

    // Init markdown converter
    let md_converter = markdown::converter();

    // Init libvips library
    let _app = image::init_lib(args.concurrency).expect("failed to init libvips");

    // Lauch browser
    let (tx, rx) = mpsc::channel::<BrowserRequest>(args.concurrency);
    let token = CancellationToken::new();
    let browser_handle = browser::launch(
        args.chromium_path,
        args.user_data_dir,
        args.headless,
        args.incognito,
        rx,
        token.clone(),
    )
    .await
    .expect("failed to launch browser");

    // Start server
    let semaphore = Semaphore::new(args.concurrency);
    let config = Config {
        concurrency: args.concurrency,
        timeout_ms: args.timeout_ms,
        md_converter: Arc::new(md_converter),
        max_viewport: args.max_viewport,
        image_dir: args.image_dir,
        max_frames: args.max_frames,
        semaphore: Arc::new(semaphore),
        browser_tx: tx,
    };
    let listener = TcpListener::bind(args.addr)
        .await
        .expect("failed to create TcpListener");
    tracing::info!(
        "Server started on {}",
        listener
            .local_addr()
            .expect("failed to retrieve local address")
    );
    let app = server::build(config);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    // Shut down browser
    token.cancel();
    browser_handle.await.unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
