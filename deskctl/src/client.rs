// Unix socket client for communicating with deskd
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub struct Client {
    socket_path: String,
}

impl Client {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    pub async fn send_request(&self, request: &str) -> Result<String> {
        // Connect to daemon
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .with_context(|| format!("Failed to connect to daemon at {}", self.socket_path))?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Send request
        writer
            .write_all(request.as_bytes())
            .await
            .context("Failed to send request")?;
        writer
            .write_all(b"\n")
            .await
            .context("Failed to send newline")?;
        writer.flush().await.context("Failed to flush writer")?;

        // Read response
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .await
            .context("Failed to read response")?;

        Ok(response.trim().to_string())
    }
}
