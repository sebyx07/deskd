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
        Request::FindElement { name, role } => {
            info!("Find element request: name={:?}, role={:?}", name, role);
            Response::Data {
                data: serde_json::json!({
                    "element": null
                }),
            }
        }
        Request::ClickElement { name, button } => {
            info!("Click element request: name={}, button={:?}", name, button);
            Response::Success {
                message: format!("Would click element: {}", name),
            }
        }
        Request::DoubleClickElement { name } => {
            info!("Double-click element request: name={}", name);
            Response::Success {
                message: format!("Would double-click element: {}", name),
            }
        }
        Request::TypeIntoElement {
            name,
            text,
            secure,
        } => {
            if secure.unwrap_or(false) {
                info!("Type secure into element: name={}", name);
            } else {
                info!("Type into element: name={}, text={}", name, text);
            }
            Response::Success {
                message: format!("Would type into element: {}", name),
            }
        }
        Request::FocusElement { name } => {
            info!("Focus element request: name={}", name);
            Response::Success {
                message: format!("Would focus element: {}", name),
            }
        }
        Request::GetFocusedElement => {
            info!("Get focused element request");
            Response::Data {
                data: serde_json::json!({
                    "element": null
                }),
            }
        }
        Request::KeyPress { key } => {
            info!("Key press request: {}", key);
            Response::Success {
                message: format!("Would press key: {}", key),
            }
        }
        Request::KeyCombo { combo } => {
            info!("Key combo request: {}", combo);
            Response::Success {
                message: format!("Would press key combo: {}", combo),
            }
        }
        Request::KeySequence { keys } => {
            info!("Key sequence request: {} keys", keys.len());
            Response::Success {
                message: format!("Would press {} keys", keys.len()),
            }
        }
        Request::ClickAt { x, y, button } => {
            let btn = button.as_deref().unwrap_or("left");
            info!("Click at request: ({}, {}) with button: {}", x, y, btn);
            Response::Success {
                message: format!("Would click at ({}, {}) with {}", x, y, btn),
            }
        }
        Request::Drag {
            from_x,
            from_y,
            to_x,
            to_y,
        } => {
            info!(
                "Drag request: from ({}, {}) to ({}, {})",
                from_x, from_y, to_x, to_y
            );
            Response::Success {
                message: format!("Would drag from ({}, {}) to ({}, {})", from_x, from_y, to_x, to_y),
            }
        }
        Request::Screenshot {
            region,
            include_cursor,
        } => {
            let reg = region.as_deref().unwrap_or("fullscreen");
            let cursor = include_cursor.unwrap_or(false);
            info!("Screenshot request: region={}, cursor={}", reg, cursor);
            Response::Data {
                data: serde_json::json!({
                    "screenshot": "base64_encoded_data_here"
                }),
            }
        }
        Request::DetectCompositor => {
            info!("Detect compositor request");
            Response::Data {
                data: serde_json::json!({
                    "compositor": "Unknown",
                    "is_wayland": false,
                    "has_xwayland": false
                }),
            }
        }
        Request::GetCapabilities => {
            info!("Get capabilities request");
            Response::Data {
                data: serde_json::json!({
                    "has_portal": false,
                    "has_ipc": false,
                    "has_wlr_protocols": false,
                    "supports_screenshots": false,
                    "supports_input": false,
                    "supports_clipboard": false
                }),
            }
        }
        Request::ClipboardHistory { limit } => {
            let lim = limit.unwrap_or(10);
            info!("Clipboard history request: limit={}", lim);
            Response::Data {
                data: serde_json::json!({
                    "history": []
                }),
            }
        }
    }
}
