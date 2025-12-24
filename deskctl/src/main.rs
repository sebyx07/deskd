use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::json;

mod client;
mod commands;

use client::Client;

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
    /// Check daemon status
    Status,

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Expand home directory in socket path
    let socket_path = if cli.socket.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            cli.socket.replacen("~", &home, 1)
        } else {
            cli.socket
        }
    } else {
        cli.socket
    };

    let client = Client::new(socket_path);

    match cli.command {
        Commands::Status => {
            // Try to connect to daemon
            match client
                .send_request(&json!({"type": "ListDesktops"}).to_string())
                .await
            {
                Ok(_) => {
                    println!("Daemon is running");
                    Ok(())
                }
                Err(e) => {
                    println!("Daemon is not running: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Input(cmd) => handle_input_command(&client, cmd).await,
        Commands::Desktop(cmd) => handle_desktop_command(&client, cmd).await,
        Commands::Clipboard(cmd) => handle_clipboard_command(&client, cmd).await,
        Commands::Workflow(cmd) => handle_workflow_command(&client, cmd).await,
        Commands::Query(cmd) => handle_query_command(&client, cmd).await,
    }
}

async fn handle_input_command(client: &Client, cmd: InputCommands) -> Result<()> {
    let request = match cmd {
        InputCommands::Type { text } => json!({"type": "Type", "data": {"text": text}}),
        InputCommands::TypeSecure { text } => json!({"type": "TypeSecure", "data": {"text": text}}),
        InputCommands::Click { x, y } => json!({"type": "Click", "data": {"x": x, "y": y}}),
    };

    let response = client.send_request(&request.to_string()).await?;
    println!("{}", response);
    Ok(())
}

async fn handle_desktop_command(client: &Client, cmd: DesktopCommands) -> Result<()> {
    let request = match cmd {
        DesktopCommands::List => json!({"type": "ListDesktops"}),
    };

    let response = client.send_request(&request.to_string()).await?;
    println!("{}", response);
    Ok(())
}

async fn handle_clipboard_command(client: &Client, cmd: ClipboardCommands) -> Result<()> {
    let request = match cmd {
        ClipboardCommands::Get => json!({"type": "ClipboardGet"}),
        ClipboardCommands::Set { content } => {
            json!({"type": "ClipboardSet", "data": {"content": content}})
        }
    };

    let response = client.send_request(&request.to_string()).await?;
    println!("{}", response);
    Ok(())
}

async fn handle_workflow_command(_client: &Client, cmd: WorkflowCommands) -> Result<()> {
    match cmd {
        WorkflowCommands::List => {
            println!("Workflow list not yet implemented");
            Ok(())
        }
    }
}

async fn handle_query_command(client: &Client, cmd: QueryCommands) -> Result<()> {
    let request = match cmd {
        QueryCommands::History { limit } => {
            json!({"type": "GetTaskHistory", "data": {"limit": limit}})
        }
    };

    let response = client.send_request(&request.to_string()).await?;
    println!("{}", response);
    Ok(())
}
