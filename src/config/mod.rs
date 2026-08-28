use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub telegram_token: String,
    pub bot_name: String,
    pub bot_username: String,
    pub bot_description: String,
    pub openfort_base_url: String,
    pub openfort_secret_key: String,
    pub openfort_wallet_secret: String,
    pub openfort_publishable_key: String,
    pub solana_rpc_url: String,
    pub solana_network: String,
    pub database_url: String,
    pub jupiter_api_key: String,
    pub referral_fee_bps: u16,
    pub referral_account: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            telegram_token: env::var("TELEGRAM_BOT_TOKEN")
                .context("TELEGRAM_BOT_TOKEN must be set")?,
            bot_name: env::var("TELEGRAM_BOT_NAME")
                .unwrap_or_else(|_| "Solana-kit-bot".to_string()),
            bot_username: env::var("TELEGRAM_BOT_USERNAME")
                .unwrap_or_else(|_| "@SolanaKitBot".to_string()),
            bot_description: env::var("TELEGRAM_BOT_DESCRIPTION")
                .unwrap_or_else(|_| "Non-custodial Telegram bot for Solana".to_string()),
            openfort_base_url: env::var("OPENFORT_BASE_URL")
                .unwrap_or_else(|_| "https://api.openfort.io".to_string()),
            openfort_secret_key: env::var("OPENFORT_SECRET_KEY")
                .context("OPENFORT_SECRET_KEY must be set")?,
            openfort_wallet_secret: env::var("OPENFORT_WALLET_SECRET")
                .context("OPENFORT_WALLET_SECRET must be set")?,
            // Required because withdraw already depends on Kora through this key —
            // better to fail at startup than to get a 401 on an actual withdrawal
            openfort_publishable_key: env::var("OPENFORT_PUBLISHABLE_KEY")
                .context("OPENFORT_PUBLISHABLE_KEY must be set (required for Kora/withdraw)")?,
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
            solana_network: env::var("SOLANA_NETWORK")
                .unwrap_or_else(|_| "devnet".to_string()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:./data/bot.db".to_string()),
            jupiter_api_key: env::var("JUPITER_API_KEY")
                .unwrap_or_else(|_| "".to_string()),
            referral_fee_bps: env::var("REFERRAL_FEE_BPS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .context("REFERRAL_FEE_BPS must be a number (bps, e.g. 50)")?,
            referral_account: env::var("REFERRAL_ACCOUNT").ok().filter(|s| !s.is_empty()),
        })
    }
}