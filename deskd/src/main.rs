use anyhow::{Context, Result};
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod atspi;
mod config;
mod db;
mod input;
mod ipc;
mod wayland;

use config::Config;
use db::Database;
use ipc::IpcServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "deskd=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting deskd daemon");

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;
    info!(
        "Configuration loaded: database={}, socket={}",
        config.database_path, config.socket_path
    );

    // Initialize database
    let database = Database::new(&config.database_path).context("Failed to initialize database")?;
    database
        .migrate()
        .await
        .context("Failed to run database migrations")?;

    // Set up IPC socket server
    let ipc_server = IpcServer::new(&config.socket_path).context("Failed to create IPC server")?;

    // Set up signal handlers
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    info!("deskd daemon started successfully");
    info!("Listening on: {}", config.socket_path);

    // Run the server
    tokio::select! {
        result = ipc_server.run() => {
            if let Err(e) = result {
                error!("IPC server error: {}", e);
                return Err(e);
            }
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, shutting down gracefully");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT, shutting down gracefully");
        }
    }

    // Cleanup
    info!("Cleaning up...");
    if let Err(e) = std::fs::remove_file(&config.socket_path) {
        error!("Failed to remove socket file: {}", e);
    }

    info!("deskd daemon stopped");
    Ok(())
}
