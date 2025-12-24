use anyhow::Result;

pub mod schema;
pub mod tasks;
pub mod workflows;

#[allow(dead_code)]
pub struct Database {
    // Connection pool will be added here
}

#[allow(dead_code)]
impl Database {
    pub fn new(_path: &str) -> Result<Self> {
        todo!("Initialize database connection pool")
    }

    pub async fn migrate(&self) -> Result<()> {
        todo!("Run database migrations")
    }
}
