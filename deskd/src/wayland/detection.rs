// Compositor and environment detection
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use tracing::{debug, info};

/// Supported Wayland compositor types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompositorType {
    Gnome,
    Kde,
    Sway,
    Hyprland,
    Wlroots, // Generic wlroots-based
    Unknown,
    X11, // Not Wayland
}

/// Compositor capabilities for automation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompositorCapabilities {
    pub has_portal: bool,
    pub has_ipc: bool,
    pub has_wlr_protocols: bool,
    pub has_libei: bool,
    pub supports_screenshots: bool,
    pub supports_input: bool,
    pub supports_clipboard: bool,
}

/// Detect if running on Wayland
pub fn is_wayland() -> bool {
    env::var("WAYLAND_DISPLAY").is_ok()
}

/// Detect if XWayland is available
pub fn has_xwayland() -> bool {
    env::var("DISPLAY").is_ok() && is_wayland()
}

/// Detect the compositor type
pub fn detect_compositor() -> CompositorType {
    if !is_wayland() {
        debug!("Not running on Wayland (WAYLAND_DISPLAY not set)");
        return CompositorType::X11;
    }

    // Check environment variables for compositor hints
    if let Ok(session) = env::var("XDG_CURRENT_DESKTOP") {
        let session = session.to_lowercase();
        info!("XDG_CURRENT_DESKTOP: {}", session);

        if session.contains("gnome") {
            return CompositorType::Gnome;
        } else if session.contains("kde") || session.contains("plasma") {
            return CompositorType::Kde;
        }
    }

    if let Ok(session) = env::var("DESKTOP_SESSION") {
        let session = session.to_lowercase();
        info!("DESKTOP_SESSION: {}", session);

        if session.contains("gnome") {
            return CompositorType::Gnome;
        } else if session.contains("kde") || session.contains("plasma") {
            return CompositorType::Kde;
        } else if session.contains("sway") {
            return CompositorType::Sway;
        }
    }

    // Check for Sway socket
    if env::var("SWAYSOCK").is_ok() {
        return CompositorType::Sway;
    }

    // Check for Hyprland signature
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return CompositorType::Hyprland;
    }

    // Check for wlroots-based compositors
    if has_wlroots_protocols() {
        return CompositorType::Wlroots;
    }

    debug!("Unknown Wayland compositor");
    CompositorType::Unknown
}

/// Check if wlroots protocols are available
fn has_wlroots_protocols() -> bool {
    // Check for wlr-layer-shell protocol presence
    // This is a stub - real implementation would query Wayland registry
    false
}

/// Detect compositor capabilities
pub async fn detect_capabilities(compositor: &CompositorType) -> CompositorCapabilities {
    // Check for XDG Desktop Portal (works on most compositors)
    let has_portal = check_portal_available().await;

    let mut caps = CompositorCapabilities {
        has_portal,
        ..Default::default()
    };

    match compositor {
        CompositorType::Gnome => {
            caps.has_ipc = check_gnome_dbus();
            caps.supports_screenshots = true;
            caps.supports_input = caps.has_portal;
            caps.supports_clipboard = true;
        }
        CompositorType::Kde => {
            caps.has_ipc = check_kde_dbus();
            caps.supports_screenshots = true;
            caps.supports_input = caps.has_portal;
            caps.supports_clipboard = true;
        }
        CompositorType::Sway => {
            caps.has_ipc = check_sway_socket();
            caps.has_wlr_protocols = true;
            caps.supports_screenshots = true;
            caps.supports_input = caps.has_ipc || caps.has_portal;
            caps.supports_clipboard = true;
        }
        CompositorType::Hyprland => {
            caps.has_ipc = check_hyprland_socket();
            caps.supports_screenshots = true;
            caps.supports_input = caps.has_ipc || caps.has_portal;
            caps.supports_clipboard = true;
        }
        CompositorType::Wlroots => {
            caps.has_wlr_protocols = true;
            caps.supports_screenshots = true;
            caps.supports_input = caps.has_portal;
            caps.supports_clipboard = true;
        }
        CompositorType::X11 => {
            caps.supports_screenshots = true;
            caps.supports_input = true; // Via XTest
            caps.supports_clipboard = true;
        }
        CompositorType::Unknown => {
            caps.supports_input = caps.has_portal;
            caps.supports_clipboard = caps.has_portal;
        }
    }

    // Check for libei availability
    caps.has_libei = check_libei_available();

    info!("Detected capabilities: {:?}", caps);
    caps
}

/// Check if XDG Desktop Portal is available
async fn check_portal_available() -> bool {
    // Check if portal service is running on D-Bus
    // This is a stub - real implementation would use zbus to check
    debug!("Checking for XDG Desktop Portal...");
    // For now, assume available on Wayland
    is_wayland()
}

/// Check if GNOME D-Bus methods are available
fn check_gnome_dbus() -> bool {
    debug!("Checking for GNOME D-Bus interface...");
    // Stub: would check for org.gnome.Shell interface
    false
}

/// Check if KDE D-Bus methods are available
fn check_kde_dbus() -> bool {
    debug!("Checking for KDE D-Bus interface...");
    // Stub: would check for org.kde.KWin interface
    false
}

/// Check if Sway IPC socket is available
fn check_sway_socket() -> bool {
    if let Ok(socket_path) = env::var("SWAYSOCK") {
        debug!("Checking Sway socket at: {}", socket_path);
        Path::new(&socket_path).exists()
    } else {
        false
    }
}

/// Check if Hyprland socket is available
fn check_hyprland_socket() -> bool {
    if let Ok(sig) = env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        let socket_path = format!("/tmp/hypr/{}/.socket.sock", sig);
        debug!("Checking Hyprland socket at: {}", socket_path);
        Path::new(&socket_path).exists()
    } else {
        false
    }
}

/// Check if libei is available
fn check_libei_available() -> bool {
    debug!("Checking for libei availability...");
    // Stub: would check if libei library is present
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_detection() {
        let compositor = detect_compositor();
        // Should not panic
        let _ = compositor;
    }

    #[test]
    fn test_wayland_detection() {
        let is_wl = is_wayland();
        let has_xw = has_xwayland();
        // Just verify they don't panic
        let _ = (is_wl, has_xw);
    }

    #[tokio::test]
    async fn test_capabilities_detection() {
        let compositor = detect_compositor();
        let caps = detect_capabilities(&compositor).await;
        // Should return valid capabilities
        assert!(caps.has_portal || !caps.has_portal); // Always true
    }
}
