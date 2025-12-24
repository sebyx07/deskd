// Configuration management
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_database_path")]
    pub database_path: String,

    #[serde(default = "default_socket_path")]
    pub socket_path: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default)]
    pub session_discovery: SessionDiscovery,

    #[serde(default)]
    pub input_timing: InputTiming,

    #[serde(default)]
    pub wayland: WaylandConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SessionDiscovery {
    #[default]
    Auto,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTiming {
    #[serde(default = "default_typing_delay")]
    pub typing_delay_ms: u64,

    #[serde(default = "default_click_delay")]
    pub click_delay_ms: u64,

    #[serde(default = "default_focus_timeout")]
    pub focus_timeout_ms: u64,
}

impl Default for InputTiming {
    fn default() -> Self {
        Self {
            typing_delay_ms: default_typing_delay(),
            click_delay_ms: default_click_delay(),
            focus_timeout_ms: default_focus_timeout(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaylandConfig {
    #[serde(default)]
    pub compositor: Option<String>,

    #[serde(default = "default_input_methods")]
    pub input_methods: Vec<String>,

    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: usize,

    #[serde(default = "default_retry_delay_ms")]
    pub retry_delay_ms: u64,

    #[serde(default = "default_portal_timeout_ms")]
    pub portal_permission_timeout_ms: u64,

    #[serde(default = "default_clipboard_history_size")]
    pub clipboard_history_size: usize,
}

impl Default for WaylandConfig {
    fn default() -> Self {
        Self {
            compositor: None, // Auto-detect
            input_methods: default_input_methods(),
            retry_attempts: default_retry_attempts(),
            retry_delay_ms: default_retry_delay_ms(),
            portal_permission_timeout_ms: default_portal_timeout_ms(),
            clipboard_history_size: default_clipboard_history_size(),
        }
    }
}

// Default value functions
fn default_database_path() -> String {
    expand_home("~/.local/share/deskd/state.db")
}

fn default_socket_path() -> String {
    expand_home("~/.local/run/deskd.sock")
}

fn default_log_level() -> String {
    std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string())
}

fn default_typing_delay() -> u64 {
    10
}

fn default_click_delay() -> u64 {
    50
}

fn default_focus_timeout() -> u64 {
    1000
}

fn default_input_methods() -> Vec<String> {
    vec![
        "portal".to_string(),
        "compositor_ipc".to_string(),
        "libei".to_string(),
        "ydotool".to_string(),
        "xtest".to_string(),
    ]
}

fn default_retry_attempts() -> usize {
    3
}

fn default_retry_delay_ms() -> u64 {
    100
}

fn default_portal_timeout_ms() -> u64 {
    30000 // 30 seconds for user to respond to permission dialog
}

fn default_clipboard_history_size() -> usize {
    100
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_path: default_database_path(),
            socket_path: default_socket_path(),
            log_level: default_log_level(),
            session_discovery: SessionDiscovery::default(),
            input_timing: InputTiming::default(),
            wayland: WaylandConfig::default(),
        }
    }
}

impl Config {
    /// Load configuration from file with fallback to defaults
    pub fn load() -> Result<Self> {
        let config_path = Self::find_config_file();

        if let Some(path) = config_path {
            info!("Loading configuration from: {}", path.display());
            Self::load_from_file(&path)
        } else {
            warn!("No configuration file found, using defaults");
            Ok(Self::default())
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let mut config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        // Expand home directory in paths
        config.database_path = expand_home(&config.database_path);
        config.socket_path = expand_home(&config.socket_path);

        // Override with environment variables
        if let Ok(db_path) = std::env::var("DESKD_DATABASE_PATH") {
            config.database_path = db_path;
        }
        if let Ok(socket_path) = std::env::var("DESKD_SOCKET_PATH") {
            config.socket_path = socket_path;
        }
        if let Ok(log_level) = std::env::var("RUST_LOG") {
            config.log_level = log_level;
        }

        config.validate()?;
        Ok(config)
    }

    /// Find configuration file in standard locations
    fn find_config_file() -> Option<PathBuf> {
        let candidates = vec![
            expand_home("~/.config/deskd/deskd.conf"),
            "/etc/deskd/deskd.conf".to_string(),
        ];

        for path in candidates {
            let path = PathBuf::from(path);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate log level
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            anyhow::bail!("Invalid log level: {}", self.log_level);
        }

        // Validate paths are not empty
        if self.database_path.is_empty() {
            anyhow::bail!("database_path cannot be empty");
        }
        if self.socket_path.is_empty() {
            anyhow::bail!("socket_path cannot be empty");
        }

        Ok(())
    }

    /// Get configuration paths for reference
    #[allow(dead_code)]
    pub fn default_paths() -> ConfigPaths {
        ConfigPaths {
            user_config: expand_home("~/.config/deskd/deskd.conf"),
            user_socket: expand_home("~/.local/run/deskd.sock"),
            user_database: expand_home("~/.local/share/deskd/state.db"),
            system_config: "/etc/deskd/deskd.conf".into(),
            system_socket: "/var/run/deskd.sock".into(),
            system_database: "/var/lib/deskd/state.db".into(),
        }
    }
}

#[allow(dead_code)]
pub struct ConfigPaths {
    pub user_config: String,
    pub user_socket: String,
    pub user_database: String,
    pub system_config: String,
    pub system_socket: String,
    pub system_database: String,
}

/// Expand ~ to home directory
fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen("~", &home, 1);
        }
    }
    path.to_string()
}
