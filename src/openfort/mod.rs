//! Openfort integration module with JWT support

use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::time::Duration;
use uuid::Uuid;

use crate::crypto::der_base64_to_pem;

#[derive(Debug, Clone)]
pub struct OpenfortClient {
    base_url: String,
    secret_key: String,
    wallet_secret: String,
    publishable_key: String,
    http_client: Client,
}

#[derive(Debug, Serialize)]
struct JwtPayload {
    iat: i64,
    nbf: i64,
    jti: String,
    uris: Vec<String>,
    #[serde(rename = "reqHash", skip_serializing_if = "Option::is_none")]
    req_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountResponse {
    pub id: String,
    pub address: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    #[serde(rename = "chainType")]
    pub chain_type: String,
}

#[derive(Debug, Deserialize)]
struct SignResponse {
    pub signature: String, // hex-encoded подпись, без префикса 0x
}

impl OpenfortClient {
    pub fn new(
        base_url: String,
        secret_key: String,
        wallet_secret: String,
        publishable_key: String,
    ) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url,
            secret_key,
            wallet_secret,
            publishable_key,
            http_client,
        }
    }

    // ------------------------------------------------------------------
    // X-Wallet-Auth JWT — используется только для /v2/accounts/backend/*
    // ------------------------------------------------------------------

    fn generate_jwt(&self, method: &str, path: &str, body: &serde_json::Value) -> Result<String> {
        let now = Utc::now().timestamp();

        let is_empty_body =
            body.is_null() || body.as_object().map(|o| o.is_empty()).unwrap_or(false);

        let req_hash = if is_empty_body {
            None
        } else {
            Some(self.compute_req_hash(body)?)
        };

        let payload = JwtPayload {
            iat: now,
            nbf: now,
            jti: Uuid::new_v4().simple().to_string(),
            uris: vec![format!("{} api.openfort.io{}", method, path)],
            req_hash,
        };

        let header = Header::new(Algorithm::ES256);
        let pem_key = der_base64_to_pem(&self.wallet_secret, "PRIVATE KEY")?;
        let key = EncodingKey::from_ec_pem(pem_key.as_bytes())?;
        let jwt = jsonwebtoken::encode(&header, &payload, &key)?;
        Ok(jwt)
    }

    fn compute_req_hash(&self, body: &serde_json::Value) -> Result<String> {
        let canonical = self.canonical_json(body);
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        Ok(hex::encode(hasher.finalize()))
    }

    /// Канонизация JSON: ключи отсортированы, строки экранированы через serde_json
    /// (совпадает с `sortKeys` + `JSON.stringify` в оригинальном walletAuth.ts).
    fn canonical_json(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Object(map) => {
                let sorted: BTreeMap<_, _> = map
                    .iter()
                    .map(|(k, v)| (k.clone(), self.canonical_json(v)))
                    .collect();
                let pairs: Vec<String> = sorted
                    .iter()
                    .map(|(k, v)| format!("{}:{}", serde_json::to_string(k).unwrap(), v))
                    .collect();
                format!("{{{}}}", pairs.join(","))
            }
            serde_json::Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| self.canonical_json(v)).collect();
                format!("[{}]", items.join(","))
            }
            serde_json::Value::String(_) => serde_json::to_string(value).unwrap(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
        }
    }

    // ------------------------------------------------------------------
    // Аутентифицированные вызовы к /v2/accounts/backend/*
    // (secret_key + X-Wallet-Auth JWT)
    // ------------------------------------------------------------------

    async fn authenticated_post<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<T> {
        let jwt = self.generate_jwt("POST", path, &body)?;
        // Отправляем именно ту строку, что была захеширована в JWT —
        // TEE проверяет сырые байты тела, не пересериализует их сам.
        let canonical_body = self.canonical_json(&body);

        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .header("X-Wallet-Auth", jwt)
            .header("Content-Type", "application/json")
            .body(canonical_body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if status.is_success() {
            Ok(serde_json::from_str(&response_text)?)
        } else {
            Err(anyhow!("Openfort API error: {} - {}", status, response_text))
        }
    }

    /// Создаёт новый backend-кошелёк на Solana (chainType: SVM)
    pub async fn create_wallet(&self, user_id: &str) -> Result<CreateAccountResponse> {
        println!("🔐 Creating backend wallet for user: {}", user_id);
        self.authenticated_post("/v2/accounts/backend", json!({ "chainType": "SVM" }))
            .await
    }

    /// Подписывает сырые байты (например, message транзакции) через backend wallet.
    /// Кодирует их в hex с префиксом 0x — именно так ожидает сервер.
    /// Возвращает hex-подпись (без префикса).
    pub async fn sign_data(&self, account_id: &str, data_bytes: &[u8]) -> Result<String> {
        let path = format!("/v2/accounts/backend/{}/sign", account_id);
        let hex_data = format!("0x{}", hex::encode(data_bytes));
        let body = json!({ "data": hex_data });

        println!("🔑 Signing data for account: {}", account_id);
        let response: SignResponse = self.authenticated_post(&path, body).await?;
        Ok(response.signature)
    }

    // ------------------------------------------------------------------
    // Kora JSON-RPC прокси (/rpc/solana/{cluster})
    // Отдельная авторизация: publishable_key, БЕЗ X-Wallet-Auth —
    // путь не подпадает под requiresWalletAuth (не содержит /accounts/backend)
    // ------------------------------------------------------------------

    pub async fn kora_request(
        &self,
        cluster: &str,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/rpc/solana/{}", cluster);
        let url = format!("{}{}", self.base_url, path);

        let body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1,
        });

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.publishable_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let response_json: serde_json::Value = response.json().await?;

        if let Some(error) = response_json.get("error") {
            return Err(anyhow!("Kora JSON-RPC error: {}", error));
        }

        if !status.is_success() {
            return Err(anyhow!("Kora HTTP error: {} - {}", status, response_json));
        }

        response_json
            .get("result")
            .cloned()
            .ok_or_else(|| anyhow!("Отсутствует 'result' в ответе Kora: {}", response_json))
    }
}