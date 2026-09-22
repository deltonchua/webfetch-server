use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn enable() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=info,tower_http=info", env!("CARGO_CRATE_NAME")).into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
