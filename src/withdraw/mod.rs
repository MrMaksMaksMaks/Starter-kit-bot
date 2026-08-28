//! Withdrawal module via Openfort backend wallet + Kora (gasless fee payer)

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::json;
use solana_sdk::{
    hash::Hash,
    message::Message,
    pubkey::Pubkey,
    signature::Signature,
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

use crate::openfort::OpenfortClient;

/// Withdraws SOL to the specified address.
/// Kora pays the fee (fee payer), not the user — gasless transaction.
///
/// Pipeline:
/// 1. Get the fee payer address from Kora
/// 2. Get a fresh blockhash from Kora
/// 3. Build a transfer transaction where the fee payer is Kora's address
/// 4. Sign the message bytes via Openfort backend wallet (user signature)
/// 5. Insert the user's signature into the correct slot (found by address, not hardcoded 0 —
///    slot 0 belongs to Kora as the fee payer)
/// 6. Send the partially signed transaction to Kora — it attaches its own signature
///    as the fee payer and broadcasts it to the network
pub async fn withdraw_sol(
    openfort: &OpenfortClient,
    account_id: &str,
    from_address: &str,
    to_address: &str,
    amount_lamports: u64,
    cluster: &str,
) -> Result<String> {
    println!("💸 Withdrawing {} lamports to {}", amount_lamports, to_address);

    // 1. Fee payer from Kora
    println!("⏳ Getting Kora fee payer...");
    let payer_result = openfort
        .kora_request(cluster, "getPayerSigner", json!({}))
        .await?;

    let signer_address = payer_result["signer_address"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing signer_address in Kora response: {}", payer_result))?
        .to_string();
    println!("✅ Kora fee payer: {}", signer_address);

    // 2. Blockhash from Kora
    println!("⏳ Getting blockhash from Kora...");
    let blockhash_result = openfort
        .kora_request(cluster, "getBlockhash", json!({}))
        .await?;

    let blockhash_str = blockhash_result["blockhash"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing blockhash in Kora response: {}", blockhash_result))?;
    let blockhash = Hash::from_str(blockhash_str)
        .map_err(|_| anyhow!("Invalid blockhash: {}", blockhash_str))?;
    println!("✅ Blockhash: {}", blockhash);

    // 3. Build transaction — fee payer is Kora, not the user
    let from_pubkey = Pubkey::from_str(from_address)
        .map_err(|_| anyhow!("Invalid sender address: {}", from_address))?;
    let to_pubkey = Pubkey::from_str(to_address)
        .map_err(|_| anyhow!("Invalid recipient address: {}", to_address))?;
    let payer_pubkey = Pubkey::from_str(&signer_address)
        .map_err(|_| anyhow!("Invalid Kora fee payer address: {}", signer_address))?;

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, amount_lamports);
    let message = Message::new_with_blockhash(&[instruction], Some(&payer_pubkey), &blockhash);
    let mut tx = Transaction::new_unsigned(message);
    // tx.signatures are already initialized with zero signatures equal to the number of required signers

    // 4. Sign the message bytes, not the whole transaction
    let message_bytes = tx.message.serialize();
    println!("⏳ Signing with Openfort Backend Wallet...");
    let signature_hex = openfort.sign_data(account_id, &message_bytes).await?;

    let sig_bytes = hex::decode(signature_hex.trim_start_matches("0x"))
        .map_err(|e| anyhow!("Failed to decode signature from hex: {}", e))?;
    let signature = Signature::try_from(sig_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid signature length: {} bytes", sig_bytes.len()))?;

    // 5. Find the user's actual index among signers — slot 0 belongs to Kora
    let signer_index = tx
        .message
        .account_keys
        .iter()
        .position(|k| *k == from_pubkey)
        .ok_or_else(|| anyhow!("User account not found among transaction signers"))?;

    tx.signatures[signer_index] = signature;
    println!("✅ User signature inserted into slot {}", signer_index);

    // 6. Serialize the partially signed transaction (Kora's slot is still empty)
    let partial_tx_bytes = bincode::serialize(&tx)?;
    let partial_tx_base64 = BASE64.encode(&partial_tx_bytes);

    // 7. Kora attaches its signature as the fee payer and sends it to the network
    println!("⏳ Kora signing and sending transaction...");
    let send_result = openfort
        .kora_request(
            cluster,
            "signAndSendTransaction",
            json!({
                "transaction": partial_tx_base64,
                "signer_key": signer_address,
            }),
        )
        .await?;

    let signature_str = send_result["signature"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing signature in Kora response: {}", send_result))?
        .to_string();

    println!("✅ Transaction confirmed! TXID: {}", signature_str);
    Ok(signature_str)
}