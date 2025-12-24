// Unix socket IPC server
use anyhow::Result;

pub mod protocol;
pub mod server;

pub use server::UnixSocketServer;

pub struct IpcServer {
    server: UnixSocketServer,
}

impl IpcServer {
    pub fn new(socket_path: &str) -> Result<Self> {
        let server = UnixSocketServer::new(socket_path)?;
        Ok(Self { server })
    }

    pub async fn run(&self) -> Result<()> {
        self.server.run().await
    }
}
