use anyhow::Result;

pub mod schema;
pub mod tasks;
pub mod workflows;

pub struct Database {
    // Connection pool will be added here
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        todo!("Initialize database connection pool")
    }

    pub async fn migrate(&self) -> Result<()> {
        todo!("Run database migrations")
    }
}
