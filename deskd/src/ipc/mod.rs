// Unix socket IPC server
use anyhow::Result;

pub mod protocol;
pub mod server;

#[allow(dead_code)]
pub struct IpcServer {
    // Unix socket server will be added here
}

#[allow(dead_code)]
impl IpcServer {
    pub async fn new(_socket_path: &str) -> Result<Self> {
        todo!("Initialize Unix socket server")
    }

    pub async fn run(&self) -> Result<()> {
        todo!("Run IPC server event loop")
    }
}
