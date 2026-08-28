//! Balance formatting module (SOL + SPL tokens) for Telegram output.
//! Does not make its own RPC requests — uses already verified functions
//! from solana.rs to keep mint addresses and query logic in one place.

use crate::jupiter::symbol_for_mint;
use crate::solana::{self, TokenBalance};
use anyhow::Result;

/// Get SOL + all SPL tokens and format them for Telegram MarkdownV2
pub async fn get_formatted_balances(rpc_url: &str, address: &str) -> Result<String> {
    let sol_balance = solana::get_balance(rpc_url, address).await?;
    let token_balances = solana::get_all_token_balances(rpc_url, address).await?;

    Ok(format_balances(sol_balance, &token_balances))
}

/// Formatting for MarkdownV2: single asterisks for bold text,
/// decimal points are escaped — required for this parse_mode
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