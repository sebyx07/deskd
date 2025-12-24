// Unix socket client for communicating with deskd
use anyhow::Result;

pub struct Client {
    socket_path: String,
}

impl Client {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    pub async fn send_request(&self, request: &str) -> Result<String> {
        todo!("Send request to daemon via Unix socket")
    }
}
