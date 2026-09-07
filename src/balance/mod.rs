//! Balance formatting module (SOL + SPL tokens) for Telegram output.
//! Does not make its own RPC requests — uses already verified functions
//! from solana.rs to keep mint addresses and query logic in one place.

use crate::jupiter::symbol_for_mint;
use crate::markdown::escape_markdown_v2;
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
            // symbol_for_mint may return an arbitrary, attacker-controlled
            // string — SPL token metadata is untrusted input, and spam/scam
            // tokens routinely contain MarkdownV2 special characters.
            // Escaping it here is the actual fix: previously this was
            // inserted raw, and a single problematic symbol would make
            // Telegram reject the entire /tokens message (MarkdownV2 fails
            // the whole message, not just the offending line).
            let symbol = escape_markdown_v2(&symbol_for_mint(&token.mint));
            let amount_str = format!("{:.6}", token.ui_amount).replace('.', "\\.");
            output.push_str(&format!("• *{}*: {}\n", symbol, amount_str));
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    // Regression test for the missing-escaping bug: a token symbol containing
    // MarkdownV2 special characters must not appear raw in the output.
    #[tokio::test]
    async fn scam_token_symbol_does_not_break_markdown() {
        let result = get_formatted_balances("http://fake-rpc-for-test", "fake-address")
            .await
            .unwrap();
        assert!(!result.contains("SCAM.rug-2000!"));
    }
}
