// Unix socket client for communicating with deskd
use anyhow::Result;

#[allow(dead_code)]
pub struct Client {
    socket_path: String,
}

#[allow(dead_code)]
impl Client {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }

    pub async fn send_request(&self, _request: &str) -> Result<String> {
        todo!("Send request to daemon via Unix socket")
    }
}
