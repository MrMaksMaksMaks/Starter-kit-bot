//! Репозиторий для работы с пользователями

use anyhow::Result;
use sqlx::SqlitePool;

use super::models::{NewUser, User};

/// Репозиторий пользователей
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Создание нового пользователя
    pub async fn create(&self, new_user: NewUser) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (telegram_id, openfort_account_id, solana_address, wallet_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, telegram_id, openfort_account_id, solana_address, wallet_id, created_at, updated_at
            "#
        )
        .bind(new_user.telegram_id)
        .bind(new_user.openfort_account_id)
        .bind(new_user.solana_address)
        .bind(new_user.wallet_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Поиск пользователя по Telegram ID
    pub async fn find_by_telegram_id(&self, telegram_id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, telegram_id, openfort_account_id, solana_address, wallet_id, created_at, updated_at
            FROM users
            WHERE telegram_id = $1
            "#
        )
        .bind(telegram_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// Проверка существования пользователя
    pub async fn exists(&self, telegram_id: i64) -> Result<bool> {
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM users WHERE telegram_id = $1
            "#
        )
        .bind(telegram_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0 > 0)
    }
}
