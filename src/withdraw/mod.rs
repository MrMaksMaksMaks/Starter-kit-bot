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
use spl_associated_token_account::get_associated_token_address_with_program_id;
use std::str::FromStr;

use crate::openfort::OpenfortClient;
use crate::solana;

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

/// Withdraws an SPL token (legacy Token program or Token-2022) to the specified address.
/// Same gasless mechanics as `withdraw_sol` — Kora pays the network fee as fee payer —
/// the only structural difference is the instruction itself (`transfer_checked` against
/// the sender's and recipient's Associated Token Accounts, instead of a native SOL
/// transfer) and the two SPL-specific checks below.
///
/// Deliberately out of scope for this version (documented, not silently missing —
/// see SECURITY.md "Current Limitations"):
/// - Auto-creating the recipient's ATA if it doesn't exist yet. Someone would have to
///   pay the one-time rent for that new account, and unlike the ~5000-lamport network
///   fee, that cost is not something this project has decided who sponsors yet.
///   Returns a clear, actionable error instead of silently creating one.
/// - Token-2022 mints that use extensions affecting transfer amounts (transfer fees,
///   transfer hooks). `transfer_checked` does not account for those; only plain
///   Token-2022 mints and legacy SPL Token mints are supported here.
pub async fn withdraw_spl_token(
    openfort: &OpenfortClient,
    rpc_url: &str,
    account_id: &str,
    from_address: &str,
    to_address: &str,
    mint: &str,
    amount_raw: u64,
    decimals: u8,
    cluster: &str,
) -> Result<String> {
    println!(
        "💸 Withdrawing {} raw units of {} to {}",
        amount_raw, mint, to_address
    );

    let from_pubkey = Pubkey::from_str(from_address)
        .map_err(|_| anyhow!("Invalid sender address: {}", from_address))?;
    let to_pubkey = Pubkey::from_str(to_address)
        .map_err(|_| anyhow!("Invalid recipient address: {}", to_address))?;
    let mint_pubkey =
        Pubkey::from_str(mint).map_err(|_| anyhow!("Invalid mint address: {}", mint))?;

    // 1. Which token program owns this mint — legacy SPL Token or Token-2022.
    //    Needed before we can derive the right Associated Token Accounts or build
    //    a valid transfer_checked instruction.
    println!("⏳ Resolving token program for mint...");
    let token_program_str = solana::resolve_token_program(rpc_url, mint).await?;
    let token_program_id = Pubkey::from_str(&token_program_str)
        .map_err(|_| anyhow!("Invalid token program id returned: {}", token_program_str))?;
    println!("✅ Token program: {}", token_program_str);

    // 2. Derive both ATAs — deterministic, no RPC call needed.
    let source_ata =
        get_associated_token_address_with_program_id(&from_pubkey, &mint_pubkey, &token_program_id);
    let destination_ata =
        get_associated_token_address_with_program_id(&to_pubkey, &mint_pubkey, &token_program_id);

    // 3. Recipient ATA must already exist — see the doc comment above for why this
    //    project doesn't auto-create it yet. Fail fast with a clear message instead
    //    of wasting a blockhash fetch and a signing round-trip on a transaction that
    //    would be rejected on-chain anyway.
    println!("⏳ Checking recipient's token account exists...");
    if !solana::account_exists(rpc_url, &destination_ata.to_string()).await? {
        return Err(anyhow!(
            "Recipient has no token account for this mint yet (expected at {}). \
             This project does not auto-create recipient token accounts — \
             ask the recipient to receive any amount of this token first, \
             or use a wallet that creates the account for them.",
            destination_ata
        ));
    }

    // 4. Fee payer from Kora — same call as withdraw_sol
    println!("⏳ Getting Kora fee payer...");
    let payer_result = openfort
        .kora_request(cluster, "getPayerSigner", json!({}))
        .await?;

    let signer_address = payer_result["signer_address"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing signer_address in Kora response: {}", payer_result))?
        .to_string();
    let payer_pubkey = Pubkey::from_str(&signer_address)
        .map_err(|_| anyhow!("Invalid Kora fee payer address: {}", signer_address))?;
    println!("✅ Kora fee payer: {}", signer_address);

    // 5. Blockhash from Kora
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

    // 6. Build the transfer_checked instruction — validates mint + decimals match,
    //    unlike the legacy `transfer` instruction. Authority is the user's wallet
    //    (from_pubkey), fee payer is Kora's address, exactly like withdraw_sol.
    let instruction = spl_token::instruction::transfer_checked(
        &token_program_id,
        &source_ata,
        &mint_pubkey,
        &destination_ata,
        &from_pubkey,
        &[],
        amount_raw,
        decimals,
    )?;

    let message = Message::new_with_blockhash(&[instruction], Some(&payer_pubkey), &blockhash);
    let mut tx = Transaction::new_unsigned(message);

    // 7. Sign the message bytes, not the whole transaction — same pattern as withdraw_sol
    let message_bytes = tx.message.serialize();
    println!("⏳ Signing with Openfort Backend Wallet...");
    let signature_hex = openfort.sign_data(account_id, &message_bytes).await?;

    let sig_bytes = hex::decode(signature_hex.trim_start_matches("0x"))
        .map_err(|e| anyhow!("Failed to decode signature from hex: {}", e))?;
    let signature = Signature::try_from(sig_bytes.as_slice())
        .map_err(|_| anyhow!("Invalid signature length: {} bytes", sig_bytes.len()))?;

    // 8. Find the user's actual index among signers — slot 0 belongs to Kora
    let signer_index = tx
        .message
        .account_keys
        .iter()
        .position(|k| *k == from_pubkey)
        .ok_or_else(|| anyhow!("User account not found among transaction signers"))?;

    tx.signatures[signer_index] = signature;
    println!("✅ User signature inserted into slot {}", signer_index);

    // 9. Serialize the partially signed transaction (Kora's slot is still empty)
    let partial_tx_bytes = bincode::serialize(&tx)?;
    let partial_tx_base64 = BASE64.encode(&partial_tx_bytes);

    // 10. Kora attaches its signature as the fee payer and sends it to the network
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
