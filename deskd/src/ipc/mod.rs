// Unix socket IPC server
use anyhow::Result;

pub mod protocol;
pub mod server;

pub struct IpcServer {
    // Unix socket server will be added here
}

impl IpcServer {
    pub async fn new(socket_path: &str) -> Result<Self> {
        todo!("Initialize Unix socket server")
    }

    pub async fn run(&self) -> Result<()> {
        todo!("Run IPC server event loop")
    }
}
