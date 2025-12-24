// Wayland support and multi-input methods
use anyhow::Result;
use tracing::info;

pub mod clipboard;
pub mod compositor;
pub mod detection;
pub mod input;
pub mod portal;
pub mod screenshot;

/// Wayland client for desktop automation
#[allow(dead_code)]
pub struct WaylandClient {
    compositor: detection::CompositorType,
    capabilities: detection::CompositorCapabilities,
}

#[allow(dead_code)]
impl WaylandClient {
    /// Create a new Wayland client with auto-detection
    pub async fn new() -> Result<Self> {
        info!("Initializing Wayland client...");

        let compositor = detection::detect_compositor();
        let capabilities = detection::detect_capabilities(&compositor).await;

        info!("Detected compositor: {:?}", compositor);
        info!("Capabilities: {:?}", capabilities);

        Ok(Self {
            compositor,
            capabilities,
        })
    }

    /// Get the detected compositor type
    pub fn compositor(&self) -> &detection::CompositorType {
        &self.compositor
    }

    /// Get compositor capabilities
    pub fn capabilities(&self) -> &detection::CompositorCapabilities {
        &self.capabilities
    }

    /// Check if running on Wayland
    pub fn is_wayland() -> bool {
        detection::is_wayland()
    }

    /// Check if XWayland is available
    pub fn has_xwayland() -> bool {
        detection::has_xwayland()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wayland_detection() {
        let is_wayland = WaylandClient::is_wayland();
        // Just verify it doesn't panic
        let _ = is_wayland;
    }

    #[test]
    fn test_xwayland_detection() {
        let has_xwayland = WaylandClient::has_xwayland();
        // Just verify it doesn't panic
        let _ = has_xwayland;
    }
}
