use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod atspi;
mod wayland;
mod input;
mod ipc;
mod config;

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
