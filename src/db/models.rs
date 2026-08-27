//! Модели данных для базы данных

use sqlx::FromRow;

/// Модель пользователя
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

/// Модель для создания пользователя
#[derive(Debug, Clone)]
pub struct NewUser {
    pub telegram_id: i64,
    pub openfort_account_id: String,
    pub solana_address: String,
    pub wallet_id: String,
}
