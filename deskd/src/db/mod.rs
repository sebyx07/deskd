use anyhow::{Context, Result};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use tracing::info;

pub mod schema;
pub mod tasks;
pub mod workflows;

type DbPool = Pool<SqliteConnectionManager>;

pub struct Database {
    pool: DbPool,
}

impl Database {
    /// Create a new database connection pool
    pub fn new(path: &str) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create database directory: {}", parent.display())
            })?;
        }

        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::builder()
            .max_size(15)
            .build(manager)
            .context("Failed to create connection pool")?;

        // Enable foreign keys and WAL mode for better concurrency
        let conn = pool.get().context("Failed to get connection from pool")?;
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA temp_store = MEMORY;
             PRAGMA mmap_size = 30000000000;",
        )
        .context("Failed to set database pragmas")?;

        info!("Database connection pool created: {}", path);

        Ok(Self { pool })
    }

    /// Get the current schema version
    pub fn get_schema_version(&self) -> Result<i32> {
        let conn = self.pool.get()?;

        // Check if schema_version table exists
        let table_exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='schema_version'",
            [],
            |row| row.get(0),
        )?;

        if !table_exists {
            return Ok(0);
        }

        let version: i32 =
            conn.query_row("SELECT MAX(version) FROM schema_version", [], |row| {
                row.get(0).or(Ok(0))
            })?;

        Ok(version)
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        let current_version = self.get_schema_version()?;
        info!("Current schema version: {}", current_version);

        if current_version == 0 {
            info!("Initializing database schema...");
            self.apply_initial_schema()?;
        }

        // Future migrations would go here
        // if current_version < 2 { self.apply_migration_2()?; }

        let new_version = self.get_schema_version()?;
        info!("Schema migration complete. New version: {}", new_version);

        Ok(())
    }

    /// Apply the initial database schema
    fn apply_initial_schema(&self) -> Result<()> {
        let conn = self.pool.get()?;

        // Read the migration SQL from the migrations directory
        let migration_sql = include_str!("../../../migrations/001_initial_schema.sql");

        conn.execute_batch(migration_sql)
            .context("Failed to apply initial schema")?;

        info!("Initial schema applied successfully");
        Ok(())
    }

    /// Backup the database to a file
    #[allow(dead_code)]
    pub fn backup(&self, backup_path: &str) -> Result<()> {
        let conn = self.pool.get()?;

        if let Some(parent) = Path::new(backup_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut backup_conn = rusqlite::Connection::open(backup_path)?;
        let backup = rusqlite::backup::Backup::new(&conn, &mut backup_conn)?;

        backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;

        info!("Database backed up to: {}", backup_path);
        Ok(())
    }

    /// Get a connection from the pool
    #[allow(dead_code)]
    pub fn get_conn(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool.get().context("Failed to get database connection")
    }

    /// Execute a raw SQL query (for CLI db commands)
    #[allow(dead_code)]
    pub fn execute_query(&self, sql: &str) -> Result<Vec<Vec<String>>> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare(sql)?;

        let column_count = stmt.column_count();
        let mut results = Vec::new();

        // Add header row
        let headers: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
            .collect();
        results.push(headers);

        // Add data rows
        let rows = stmt.query_map([], |row| {
            let mut row_data = Vec::new();
            for i in 0..column_count {
                let value: Result<String, rusqlite::Error> = row.get(i);
                row_data.push(value.unwrap_or_else(|_| "NULL".to_string()));
            }
            Ok(row_data)
        })?;

        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }
}
