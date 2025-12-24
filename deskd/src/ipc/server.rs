// Unix socket server implementation
use super::protocol::{Request, Response};
use anyhow::{Context, Result};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{error, info};

pub struct UnixSocketServer {
    listener: UnixListener,
}

impl UnixSocketServer {
    pub fn new(socket_path: &str) -> Result<Self> {
        // Remove existing socket file if it exists
        if Path::new(socket_path).exists() {
            std::fs::remove_file(socket_path)
                .with_context(|| format!("Failed to remove existing socket: {}", socket_path))?;
        }

        // Ensure parent directory exists
        if let Some(parent) = Path::new(socket_path).parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create socket directory: {}", parent.display())
            })?;
        }

        let listener = UnixListener::bind(socket_path)
            .with_context(|| format!("Failed to bind Unix socket: {}", socket_path))?;

        info!("Unix socket server listening on: {}", socket_path);

        Ok(Self { listener })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            match self.listener.accept().await {
                Ok((stream, _addr)) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream).await {
                            error!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

async fn handle_client(stream: UnixStream) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .await
            .context("Failed to read from socket")?;

        if n == 0 {
            // Client disconnected
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse request
        let response = match serde_json::from_str::<Request>(line) {
            Ok(request) => process_request(request).await,
            Err(e) => Response::Error {
                error: format!("Invalid request: {}", e),
            },
        };

        // Send response
        let response_json = serde_json::to_string(&response)?;
        writer
            .write_all(response_json.as_bytes())
            .await
            .context("Failed to write response")?;
        writer
            .write_all(b"\n")
            .await
            .context("Failed to write newline")?;
        writer.flush().await.context("Failed to flush writer")?;
    }

    Ok(())
}

async fn process_request(request: Request) -> Response {
    match request {
        Request::Type { text } => {
            info!("Type request: {}", text);
            Response::Success {
                message: format!("Would type: {}", text),
            }
        }
        Request::TypeSecure { .. } => {
            info!("Type secure request");
            Response::Success {
                message: "Would type securely".to_string(),
            }
        }
        Request::Click { x, y } => {
            info!("Click request: ({}, {})", x, y);
            Response::Success {
                message: format!("Would click at ({}, {})", x, y),
            }
        }
        Request::Focus { element_id } => {
            info!("Focus request: {}", element_id);
            Response::Success {
                message: format!("Would focus: {}", element_id),
            }
        }
        Request::ListDesktops => {
            info!("List desktops request");
            Response::Data {
                data: serde_json::json!({
                    "desktops": []
                }),
            }
        }
        Request::ClipboardGet => {
            info!("Clipboard get request");
            Response::Data {
                data: serde_json::json!({
                    "content": ""
                }),
            }
        }
        Request::ClipboardSet { content } => {
            info!("Clipboard set request");
            Response::Success {
                message: format!("Would set clipboard to: {}", content),
            }
        }
        Request::GetTaskHistory { limit } => {
            info!("Get task history request: {:?}", limit);
            Response::Data {
                data: serde_json::json!({
                    "tasks": []
                }),
            }
        }
    }
}
