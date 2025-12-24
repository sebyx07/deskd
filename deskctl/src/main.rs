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

    /// AT-SPI Element operations
    #[command(subcommand)]
    Element(ElementCommands),
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

#[derive(Subcommand)]
enum ElementCommands {
    /// Find an element by name or role
    Find {
        /// Element name to search for
        #[arg(short, long)]
        name: Option<String>,

        /// Element role to search for
        #[arg(short, long)]
        role: Option<String>,
    },

    /// Click an element by name
    Click {
        /// Element name
        name: String,

        /// Mouse button (left, right, middle)
        #[arg(short, long, default_value = "left")]
        button: String,
    },

    /// Double-click an element by name
    DoubleClick {
        /// Element name
        name: String,
    },

    /// Type text into an element
    Type {
        /// Element name
        name: String,

        /// Text to type
        text: String,

        /// Type securely (no logging)
        #[arg(short, long)]
        secure: bool,
    },

    /// Focus an element by name
    Focus {
        /// Element name
        name: String,
    },

    /// Get the currently focused element
    GetFocused,
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
        Commands::Element(cmd) => handle_element_command(&client, cmd).await,
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

async fn handle_element_command(client: &Client, cmd: ElementCommands) -> Result<()> {
    let request = match cmd {
        ElementCommands::Find { name, role } => {
            json!({"type": "FindElement", "data": {"name": name, "role": role}})
        }
        ElementCommands::Click { name, button } => {
            json!({"type": "ClickElement", "data": {"name": name, "button": Some(button)}})
        }
        ElementCommands::DoubleClick { name } => {
            json!({"type": "DoubleClickElement", "data": {"name": name}})
        }
        ElementCommands::Type {
            name,
            text,
            secure,
        } => {
            json!({"type": "TypeIntoElement", "data": {"name": name, "text": text, "secure": Some(secure)}})
        }
        ElementCommands::Focus { name } => {
            json!({"type": "FocusElement", "data": {"name": name}})
        }
        ElementCommands::GetFocused => json!({"type": "GetFocusedElement"}),
    };

    let response = client.send_request(&request.to_string()).await?;
    println!("{}", response);
    Ok(())
}
