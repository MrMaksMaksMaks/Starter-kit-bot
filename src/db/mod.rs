//! SQLite database module

pub mod models;
pub mod repository;

use anyhow::Result;
use sqlx::sqlite::SqlitePool;

/// Initialize database connection pool
pub async fn init_pool(database_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(database_url).await?;
    
    // Create tables
    create_tables(&pool).await?;
    
    Ok(pool)
}

/// Create database tables
async fn create_tables(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            telegram_id INTEGER UNIQUE NOT NULL,
            openfort_account_id TEXT NOT NULL,
            solana_address TEXT NOT NULL,
            wallet_id TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create indexes for fast lookups
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_users_telegram_id ON users(telegram_id)
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}