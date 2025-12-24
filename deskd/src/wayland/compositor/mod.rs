// Compositor-specific IPC clients
pub mod sway;
pub mod hyprland;
pub mod gnome;
pub mod kde;

use anyhow::Result;
use crate::wayland::detection::CompositorType;

/// Trait for compositor-specific operations
#[allow(dead_code)]
pub trait CompositorClient {
    fn compositor_type(&self) -> CompositorType;
    fn type_text(&self, text: &str) -> impl std::future::Future<Output = Result<()>> + Send;
    fn click(&self, x: i32, y: i32) -> impl std::future::Future<Output = Result<()>> + Send;
    fn screenshot(&self) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;
}
