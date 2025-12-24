use anyhow::Result;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod atspi;
mod config;
mod db;
mod input;
mod ipc;
mod wayland;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "deskd=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting deskd daemon");

    // TODO: Initialize daemon components
    // - Load configuration
    // - Initialize database
    // - Set up IPC socket
    // - Initialize AT-SPI connection
    // - Detect Wayland compositor
    // - Start main event loop

    Ok(())
}
