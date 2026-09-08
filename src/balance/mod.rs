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
            // symbol_for_mint never reads on-chain token metadata — for anything
            // outside the five hardcoded symbols it falls back to a truncated
            // "XXXX...YYYY" form of the mint address itself. Base58 mint
            // addresses can't contain MarkdownV2 special characters, but that
            // "..." fallback format always inserts three literal periods, which
            // MarkdownV2 does require escaping. Escaping here is the actual fix:
            // previously this was inserted raw, and Telegram rejects the entire
            // /tokens message over a single unescaped period (MarkdownV2 fails
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

    // Regression test for the missing-escaping bug.
    //
    // format_balances is the pure, network-free function that actually contains
    // the escaping logic — test it directly instead of going through
    // get_formatted_balances, which makes a real RPC call and can't be exercised
    // with a fake URL (that previously made this test either panic on a network
    // error or never reach the assertion it was meant to check).
    //
    // symbol_for_mint never returns free-form, attacker-controlled text: for any
    // mint outside the five hardcoded symbols (SOL/USDC/USDT/wBTC/wETH) it falls
    // back to a truncated "XXXX...YYYY" form of the mint address itself. Base58
    // mint addresses can't contain MarkdownV2 special characters — but the "..."
    // separator the function inserts is three literal periods, which MarkdownV2
    // does require escaping. That's the real, reachable case this test guards,
    // rather than a hypothetical scam-symbol string that symbol_for_mint could
    // never actually produce.
    #[test]
    fn unknown_mint_symbol_is_escaped() {
        let tokens = vec![TokenBalance {
            mint: "ScamMint1111111111111111111111111111111111".to_string(),
            amount_raw: "1000000".to_string(),
            decimals: 6,
            ui_amount: 1.0,
        }];

        let result = format_balances(0.5, &tokens);

        // symbol_for_mint renders this as "Scam...1111" — the raw, unescaped
        // form must not appear in the output.
        assert!(!result.contains("Scam...1111"));
        // The escaped form (a backslash before each literal period) must.
        assert!(result.contains("Scam\\.\\.\\.1111"));
    }
}
