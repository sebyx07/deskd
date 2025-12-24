// Configuration management
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
    pub socket_path: String,
    pub log_level: String,
    pub session_discovery: SessionDiscovery,
    pub input_timing: InputTiming,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionDiscovery {
    Auto,
    Manual,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTiming {
    pub typing_delay_ms: u64,
    pub click_delay_ms: u64,
    pub focus_timeout_ms: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        todo!("Load configuration from file")
    }

    pub fn default_paths() -> ConfigPaths {
        ConfigPaths {
            user_config: "~/.config/deskd/deskd.conf".into(),
            user_socket: "~/.local/run/deskd.sock".into(),
            user_database: "~/.local/share/deskd/state.db".into(),
            system_config: "/etc/deskd/deskd.conf".into(),
            system_socket: "/var/run/deskd.sock".into(),
            system_database: "/var/lib/deskd/state.db".into(),
        }
    }
}

pub struct ConfigPaths {
    pub user_config: String,
    pub user_socket: String,
    pub user_database: String,
    pub system_config: String,
    pub system_socket: String,
    pub system_database: String,
}
