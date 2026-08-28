//! Jupiter API v2 module

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::VersionedTransaction;
use std::str::FromStr;
use std::time::Duration;

use crate::openfort::OpenfortClient;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub transaction: Option<String>,
    pub request_id: String,
    pub out_amount: Option<String>,
    pub router: Option<String>,
    pub mode: String,
    pub fee_bps: Option<u16>,
    pub fee_mint: Option<String>,
    pub platform_fee: Option<PlatformFee>,
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    // Jupiter sends this as a string (in quotes), not a number — confirmed
    // by real mainnet response
    pub last_valid_block_height: Option<String>,
    pub in_amount: Option<String>,
    pub swap_mode: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFee {
    // Confirmed by real response: field is missing when feeBps is small —
    // Jupiter doesn't always include it
    pub amount: Option<String>,
    pub fee_bps: u16,
    pub fee_mint: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteResponse {
    pub status: String,
    pub signature: String,
    pub code: i32,
    pub total_input_amount: String,
    pub total_output_amount: String,
    pub input_amount_result: String,
    pub output_amount_result: String,
    pub error: Option<String>,
}

/// Known token mint addresses on mainnet.
/// Addresses have been cross-checked with multiple independent sources (Tether, Phantom,
/// Solana Explorer) after finding a typo in USDT.
pub mod tokens {
    pub const SOL: &str = "So11111111111111111111111111111111111111112";
    pub const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    pub const USDT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
    /// Wrapped BTC (Portal/Wormhole)
    pub const WBTC: &str = "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh";
    /// Wrapped Ether (Wormhole)
    pub const WETH: &str = "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs";
}

/// Resolves a symbol (SOL/USDC/USDT/wBTC/wETH, case-insensitive) to a mint address,
/// or returns the input as-is — assuming it's already a raw mint address
pub fn resolve_token_mint(input: &str) -> String {
    match input.to_uppercase().as_str() {
        "SOL" => tokens::SOL.to_string(),
        "USDC" => tokens::USDC.to_string(),
        "USDT" => tokens::USDT.to_string(),
        "WBTC" => tokens::WBTC.to_string(),
        "WETH" => tokens::WETH.to_string(),
        _ => input.to_string(),
    }
}

/// Known decimals for commonly used tokens — avoids unnecessary RPC calls.
/// Values for wBTC/wETH follow Wormhole bridge convention (8 decimals) —
/// before trading these tokens, it's recommended to verify via solana::get_token_decimals.
pub fn known_decimals(mint: &str) -> Option<u8> {
    match mint {
        m if m == tokens::SOL => Some(9),
        m if m == tokens::USDC => Some(6),
        m if m == tokens::USDT => Some(6),
        m if m == tokens::WBTC => Some(8),
        m if m == tokens::WETH => Some(8),
        _ => None,
    }
}

/// Human-readable symbol for a mint address.
/// For wrapped SOL (same mint used as "native" SOL in swaps)
/// displays "wSOL" to avoid confusion with native SOL balance in reports.
pub fn symbol_for_mint(mint: &str) -> String {
    match mint {
        m if m == tokens::SOL => "wSOL".to_string(),
        m if m == tokens::USDC => "USDC".to_string(),
        m if m == tokens::USDT => "USDT".to_string(),
        m if m == tokens::WBTC => "wBTC".to_string(),
        m if m == tokens::WETH => "wETH".to_string(),
        _ => {
            if mint.len() > 8 {
                format!("{}...{}", &mint[..4], &mint[mint.len() - 4..])
            } else {
                mint.to_string()
            }
        }
    }
}

/// Human-readable explanation of Jupiter error codes for aggregator routers
/// (Metis/Dflow/OKX — what we use with excludeRouters=jupiterz)
fn describe_jupiter_error(code: i32, raw_message: &str) -> String {
    match code {
        1 => "Insufficient funds for this operation".to_string(),
        2 => "Insufficient SOL to pay network fees".to_string(),
        3 => "Amount is too small for this swap mode".to_string(),
        _ => format!("Jupiter error ({}): {}", code, raw_message),
    }
}

/// Get order (quote + unsigned transaction)
pub async fn get_order(
    api_key: &str,
    input_mint: &str,
    output_mint: &str,
    amount: u64,
    taker: &str,
    slippage_bps: Option<u16>,
    exclude_jupiterz: bool,
    referral_account: Option<&str>,
    referral_fee_bps: Option<u16>,
) -> Result<OrderResponse> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let amount_str = amount.to_string();
    let slippage_str = slippage_bps.map(|s| s.to_string());
    let referral_fee_str = referral_fee_bps.map(|f| f.to_string());

    let mut params = vec![
        ("inputMint", input_mint),
        ("outputMint", output_mint),
        ("amount", &amount_str),
        ("taker", taker),
    ];

    if let Some(ref slippage) = slippage_str {
        params.push(("slippageBps", slippage.as_str()));
    }

    if exclude_jupiterz {
        params.push(("excludeRouters", "jupiterz"));
    }

    // Referral fee — both parameters are needed together, otherwise it's pointless
    if let (Some(account), Some(ref fee)) = (referral_account, &referral_fee_str) {
        params.push(("referralAccount", account));
        params.push(("referralFee", fee.as_str()));
    }

    let url = format!(
        "https://api.jup.ag/swap/v2/order?{}",
        serde_urlencoded::to_string(params)?
    );

    println!("📡 Getting order from: {}", url);

    let response = client
        .get(&url)
        .header("x-api-key", api_key)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    println!("📥 Response status: {}", status);
    println!("📄 Raw response: {}", response_text);

    if !status.is_success() {
        return Err(anyhow!("Jupiter API error: {} - {}", status, response_text));
    }

    let order: OrderResponse = serde_json::from_str(&response_text)?;

    if let Some(error_code) = order.error_code {
        println!("⚠️ Order error {}: {:?}", error_code, order.error_message);
    }

    if let Some(ref tx) = order.transaction {
        println!(
            "✅ Order received: {} -> {:?} via {:?}",
            amount, order.out_amount, order.router
        );
        println!("📦 Transaction length: {} bytes", tx.len());
    } else {
        println!("ℹ️ Quote only (no transaction)");
    }

    Ok(order)
}

/// Signs Jupiter transaction via Openfort backend wallet
pub async fn sign_jupiter_order(
    openfort: &OpenfortClient,
    account_id: &str,
    account_address: &str,
    order_transaction_b64: &str,
) -> Result<String> {
    let tx_bytes = BASE64.decode(order_transaction_b64)?;
    let mut tx: VersionedTransaction = bincode::deserialize(&tx_bytes)?;

    let message_bytes = tx.message.serialize();

    let signature_hex = openfort.sign_data(account_id, &message_bytes).await?;

    let sig_bytes = hex::decode(signature_hex.trim_start_matches("0x"))?;
    let signature = Signature::try_from(sig_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid signature length"))?;

    let our_pubkey = Pubkey::from_str(account_address)?;
    let signer_index = tx
        .message
        .static_account_keys()
        .iter()
        .position(|k| *k == our_pubkey)
        .ok_or_else(|| anyhow!("Account not found in transaction signers"))?;

    tx.signatures[signer_index] = signature;

    let signed_bytes = bincode::serialize(&tx)?;
    Ok(BASE64.encode(&signed_bytes))
}

/// Execute signed transaction
pub async fn execute_swap(
    api_key: &str,
    signed_transaction: &str,
    request_id: &str,
) -> Result<ExecuteResponse> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = "https://api.jup.ag/swap/v2/execute";

    let request_body = json!({
        "signedTransaction": signed_transaction,
        "requestId": request_id,
    });

    println!("📡 Executing swap...");

    let response = client
        .post(url)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    println!("📥 Response status: {}", status);
    println!("📄 Response: {}", response_text);

    if !status.is_success() {
        return Err(anyhow!("Execute error: {} - {}", status, response_text));
    }

    let result: ExecuteResponse = serde_json::from_str(&response_text)?;

    if result.status == "Success" {
        println!("✅ Swap executed! TX: {}", result.signature);
    } else {
        println!("❌ Swap failed: {:?}", result.error);
    }

    Ok(result)
}

/// Common pipeline: order → sign via Openfort → execute.
/// Used for both /buy and /sell — all complexity in one place.
pub async fn perform_swap(
    openfort: &OpenfortClient,
    jupiter_api_key: &str,
    account_id: &str,
    account_address: &str,
    input_mint: &str,
    output_mint: &str,
    amount: u64,
    slippage_bps: u16,
    referral_account: Option<&str>,
    referral_fee_bps: Option<u16>,
) -> Result<ExecuteResponse> {
    let order = get_order(
        jupiter_api_key,
        input_mint,
        output_mint,
        amount,
        account_address,
        Some(slippage_bps),
        true,
        referral_account,
        referral_fee_bps,
    )
    .await?;

    let tx_b64 = order.transaction.clone().ok_or_else(|| {
        let code = order.error_code.unwrap_or(-1);
        let msg = order
            .error_message
            .clone()
            .unwrap_or_else(|| "unknown error".to_string());
        anyhow!(describe_jupiter_error(code, &msg))
    })?;

    let signed_tx = sign_jupiter_order(openfort, account_id, account_address, &tx_b64).await?;
    let result = execute_swap(jupiter_api_key, &signed_tx, &order.request_id).await?;

    if result.status != "Success" {
        return Err(anyhow!(
            "Swap failed: {}",
            result
                .error
                .clone()
                .unwrap_or_else(|| "unknown error".to_string())
        ));
    }

    Ok(result)
}