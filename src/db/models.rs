//! Database models

use sqlx::FromRow;

/// User model
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub telegram_id: i64,
    pub openfort_account_id: String,
    pub solana_address: String,
    pub wallet_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Model for creating a new user
#[derive(Debug, Clone)]
pub struct NewUser {
    pub telegram_id: i64,
    pub openfort_account_id: String,
    pub solana_address: String,
    pub wallet_id: String,
}