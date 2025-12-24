use anyhow::Result;
use clap::{Parser, Subcommand};

mod client;
mod commands;

#[derive(Parser)]
#[command(name = "deskctl")]
#[command(about = "Control deskd daemon for Linux desktop automation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to Unix socket
    #[arg(short, long, default_value = "~/.local/run/deskd.sock")]
    socket: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Input operations
    #[command(subcommand)]
    Input(InputCommands),

    /// Desktop operations
    #[command(subcommand)]
    Desktop(DesktopCommands),

    /// Clipboard operations
    #[command(subcommand)]
    Clipboard(ClipboardCommands),

    /// Workflow operations
    #[command(subcommand)]
    Workflow(WorkflowCommands),

    /// Database queries
    #[command(subcommand)]
    Query(QueryCommands),
}

#[derive(Subcommand)]
enum InputCommands {
    /// Type text
    Type { text: String },

    /// Type text securely (no logging)
    TypeSecure { text: String },

    /// Click at coordinates
    Click { x: i32, y: i32 },
}

#[derive(Subcommand)]
enum DesktopCommands {
    /// List all desktop sessions
    List,
}

#[derive(Subcommand)]
enum ClipboardCommands {
    /// Get clipboard content
    Get,

    /// Set clipboard content
    Set { content: String },
}

#[derive(Subcommand)]
enum WorkflowCommands {
    /// List workflows
    List,
}

#[derive(Subcommand)]
enum QueryCommands {
    /// Get task history
    History { limit: Option<usize> },
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    // TODO: Connect to daemon via Unix socket and send command

    Ok(())
}
