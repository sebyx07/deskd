// AT-SPI accessibility integration
use anyhow::{Context, Result};
use atspi::connection::AccessibilityConnection;
use tracing::info;

pub mod cache;
pub mod element;
pub mod focus;
pub mod input;

/// AT-SPI client for desktop automation
#[allow(dead_code)]
pub struct AtSpiClient {
    connection: AccessibilityConnection,
}

#[allow(dead_code)]
impl AtSpiClient {
    /// Create a new AT-SPI client connection
    pub async fn new() -> Result<Self> {
        info!("Connecting to AT-SPI accessibility bus...");

        let connection = AccessibilityConnection::new()
            .await
            .context("Failed to connect to AT-SPI accessibility bus")?;

        info!("Successfully connected to AT-SPI");

        Ok(Self { connection })
    }

    /// Get the accessibility connection
    pub fn connection(&self) -> &AccessibilityConnection {
        &self.connection
    }

    /// Check if AT-SPI is available and responsive
    pub async fn is_available(&self) -> bool {
        // For now, if we have a connection, we consider it available
        info!("AT-SPI connection is available");
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires AT-SPI bus to be running
    async fn test_connection() {
        let client = AtSpiClient::new().await;
        assert!(client.is_ok());
    }
}
