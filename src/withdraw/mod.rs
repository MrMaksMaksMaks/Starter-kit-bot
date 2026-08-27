//! Модуль для вывода средств через Openfort backend wallet + Kora (gasless fee payer)

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

/// Выводит SOL на указанный адрес.
/// Комиссию платит Kora (fee payer), а не сам пользователь — gasless-транзакция.
///
/// Пайплайн:
/// 1. Получаем адрес fee payer'а от Kora
/// 2. Получаем свежий blockhash от Kora
/// 3. Строим транзакцию перевода, где fee payer — адрес Kora
/// 4. Подписываем message-байты через Openfort backend wallet (подпись пользователя)
/// 5. Вставляем подпись пользователя в правильный слот (найденный по адресу, не хардкод 0 —
///    слот 0 принадлежит Kora как fee payer)
/// 6. Отправляем частично подписанную транзакцию в Kora — она досоединяет свою подпись
///    как fee payer и broadcast'ит в сеть
pub async fn withdraw_sol(
    openfort: &OpenfortClient,
    account_id: &str,
    from_address: &str,
    to_address: &str,
    amount_lamports: u64,
    cluster: &str,
) -> Result<String> {
    println!("💸 Withdrawing {} lamports to {}", amount_lamports, to_address);

    // 1. Fee payer от Kora
    println!("⏳ Getting Kora fee payer...");
    let payer_result = openfort
        .kora_request(cluster, "getPayerSigner", json!({}))
        .await?;

    let signer_address = payer_result["signer_address"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing signer_address in Kora response: {}", payer_result))?
        .to_string();
    println!("✅ Kora fee payer: {}", signer_address);

    // 2. Blockhash от Kora
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

    // 3. Строим транзакцию — fee payer это Kora, не пользователь
    let from_pubkey = Pubkey::from_str(from_address)
        .map_err(|_| anyhow!("Некорректный адрес отправителя: {}", from_address))?;
    let to_pubkey = Pubkey::from_str(to_address)
        .map_err(|_| anyhow!("Некорректный адрес получателя: {}", to_address))?;
    let payer_pubkey = Pubkey::from_str(&signer_address)
        .map_err(|_| anyhow!("Некорректный адрес Kora fee payer: {}", signer_address))?;

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, amount_lamports);
    let message = Message::new_with_blockhash(&[instruction], Some(&payer_pubkey), &blockhash);
    let mut tx = Transaction::new_unsigned(message);
    // tx.signatures уже инициализированы нулевыми подписями по числу required signers

    // 4. Подписываем именно message-байты, не всю транзакцию целиком
    let message_bytes = tx.message.serialize();
    println!("⏳ Signing with Openfort Backend Wallet...");
    let signature_hex = openfort.sign_data(account_id, &message_bytes).await?;

    let sig_bytes = hex::decode(signature_hex.trim_start_matches("0x"))
        .map_err(|e| anyhow!("Не удалось декодировать подпись из hex: {}", e))?;
    let signature = Signature::try_from(sig_bytes.as_slice())
        .map_err(|_| anyhow!("Некорректная длина подписи: {} байт", sig_bytes.len()))?;

    // 5. Ищем реальный индекс пользователя среди сайнеров — слот 0 принадлежит Kora
    let signer_index = tx
        .message
        .account_keys
        .iter()
        .position(|k| *k == from_pubkey)
        .ok_or_else(|| anyhow!("Аккаунт пользователя не найден среди сайнеров транзакции"))?;

    tx.signatures[signer_index] = signature;
    println!("✅ Подпись пользователя вставлена в слот {}", signer_index);

    // 6. Сериализуем частично подписанную транзакцию (слот Kora пока пустой)
    let partial_tx_bytes = bincode::serialize(&tx)?;
    let partial_tx_base64 = BASE64.encode(&partial_tx_bytes);

    // 7. Kora досоединяет свою подпись как fee payer и отправляет в сеть
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

    println!("✅ Транзакция подтверждена! TXID: {}", signature_str);
    Ok(signature_str)
}