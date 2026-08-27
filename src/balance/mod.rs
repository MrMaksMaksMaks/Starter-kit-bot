//! Модуль форматирования балансов (SOL + SPL-токены) для вывода в Telegram.
//! Не делает собственных RPC-запросов — использует уже проверенные функции
//! из solana.rs, чтобы mint-адреса и логика запросов существовали в одном месте.

use crate::jupiter::symbol_for_mint;
use crate::solana::{self, TokenBalance};
use anyhow::Result;

/// Получить SOL + все SPL-токены и сразу отформатировать под Telegram MarkdownV2
pub async fn get_formatted_balances(rpc_url: &str, address: &str) -> Result<String> {
    let sol_balance = solana::get_balance(rpc_url, address).await?;
    let token_balances = solana::get_all_token_balances(rpc_url, address).await?;

    Ok(format_balances(sol_balance, &token_balances))
}

/// Форматирование в MarkdownV2: одинарные звёздочки для жирного текста,
/// точка в числах экранирована — обязательно для этого parse_mode
fn format_balances(sol_balance: f64, tokens: &[TokenBalance]) -> String {
    let mut output = String::from("💰 *Balance Report*\n\n");

    output.push_str(&format!(
        "*SOL*: {}\n\n",
        format!("{:.6}", sol_balance).replace('.', "\\.")
    ));

    if tokens.is_empty() {
        output.push_str("📭 _No tokens found_");
    } else {
        output.push_str("📊 *Tokens:*\n");
        for token in tokens {
            let symbol = symbol_for_mint(&token.mint);
            let amount_str = format!("{:.6}", token.ui_amount).replace('.', "\\.");
            output.push_str(&format!("• *{}*: {}\n", symbol, amount_str));
        }
    }

    output
}