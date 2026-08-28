//! Solana RPC module: SOL balance, transaction status, SPL tokens

use anyhow::{anyhow, Result};
use serde_json::json;
use std::time::Duration;

/// Legacy SPL Token Program
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
/// Token-2022 — new standard, some tokens may only exist here
const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";

#[derive(Debug, Clone)]
pub struct TokenBalance {
    pub mint: String,
    pub amount_raw: String,
    pub decimals: u8,
    pub ui_amount: f64,
}

/// Get SOL balance by address via HTTP
pub async fn get_balance(rpc_url: &str, address: &str) -> Result<f64> {
    println!("🔍 Getting balance for address: {}", address);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [address]
    });

    println!("📡 Sending RPC request to: {}", rpc_url);

    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    println!("📥 Response status: {}", status);
    println!("📄 Response: {}", response_text);

    if !status.is_success() {
        return Err(anyhow!("HTTP error: {}", status));
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(error) = response_json.get("error") {
        return Err(anyhow!("RPC error: {}", error));
    }

    let balance_lamports = response_json["result"]["value"]
        .as_u64()
        .or_else(|| response_json["result"].as_u64())
        .ok_or_else(|| anyhow!("Invalid response format"))?;

    let balance_sol = balance_lamports as f64 / 1_000_000_000.0;
    println!("💰 Balance: {} SOL", balance_sol);

    Ok(balance_sol)
}

/// Check transaction status by hash
pub async fn get_transaction_status(rpc_url: &str, txid: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSignatureStatuses",
        "params": [[txid]]
    });

    println!("🔍 Checking transaction status for: {}", txid);

    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("📄 Status response: {}", response_text);

    let json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(value) = json["result"]["value"][0].as_object() {
        if let Some(err) = value.get("err") {
            if !err.is_null() {
                return Ok("Failed".to_string());
            }
        }
        if let Some(confirmation) = value.get("confirmationStatus") {
            return Ok(confirmation.as_str().unwrap_or("unknown").to_string());
        }
        return Ok("pending".to_string());
    }

    Ok("not_found".to_string())
}

/// Gets token decimals by mint address via getTokenSupply
pub async fn get_token_decimals(rpc_url: &str, mint: &str) -> Result<u8> {
    println!("🔍 Getting decimals for mint: {}", mint);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTokenSupply",
        "params": [mint]
    });

    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    println!("📄 Decimals response: {}", response_text);

    if !status.is_success() {
        return Err(anyhow!("HTTP error: {}", status));
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(error) = response_json.get("error") {
        return Err(anyhow!(
            "RPC error while fetching decimals for {}: {}",
            mint,
            error
        ));
    }

    let decimals = response_json["result"]["value"]["decimals"]
        .as_u64()
        .ok_or_else(|| {
            anyhow!(
                "Failed to get decimals for {} — possibly not an SPL token",
                mint
            )
        })?;

    Ok(decimals as u8)
}

/// Fetches token accounts for a specific token program (SPL or Token-2022)
async fn fetch_token_accounts_for_program(
    rpc_url: &str,
    owner_address: &str,
    program_id: &str,
) -> Result<Vec<TokenBalance>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTokenAccountsByOwner",
        "params": [
            owner_address,
            { "programId": program_id },
            { "encoding": "jsonParsed" }
        ]
    });

    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        return Err(anyhow!("HTTP error: {}", status));
    }

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(error) = response_json.get("error") {
        return Err(anyhow!("RPC error: {}", error));
    }

    let accounts = response_json["result"]["value"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let mut balances = Vec::new();
    for account in accounts {
        let info = &account["account"]["data"]["parsed"]["info"];
        let mint = match info["mint"].as_str() {
            Some(m) => m.to_string(),
            None => continue,
        };
        let token_amount = &info["tokenAmount"];
        let amount_raw = token_amount["amount"].as_str().unwrap_or("0").to_string();
        let decimals = token_amount["decimals"].as_u64().unwrap_or(0) as u8;
        let ui_amount = token_amount["uiAmount"].as_f64().unwrap_or(0.0);

        if ui_amount > 0.0 {
            balances.push(TokenBalance {
                mint,
                amount_raw,
                decimals,
                ui_amount,
            });
        }
    }

    Ok(balances)
}

/// Returns all SPL token balances (including Token-2022) with non-zero balance
pub async fn get_all_token_balances(
    rpc_url: &str,
    owner_address: &str,
) -> Result<Vec<TokenBalance>> {
    println!("🔍 Getting all token balances for: {}", owner_address);

    let mut all_balances =
        fetch_token_accounts_for_program(rpc_url, owner_address, TOKEN_PROGRAM_ID).await?;

    let token_2022_balances =
        fetch_token_accounts_for_program(rpc_url, owner_address, TOKEN_2022_PROGRAM_ID).await?;

    all_balances.extend(token_2022_balances);

    println!("💎 Found {} non-zero token balances", all_balances.len());

    Ok(all_balances)
}