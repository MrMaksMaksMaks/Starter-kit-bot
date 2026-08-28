//! Cryptographic utilities for key management

use base64::{engine::general_purpose::STANDARD, Engine as _};
use pem::{Pem, encode};
use anyhow::Result;

/// Converts a DER key in base64 format to a PEM string
pub fn der_base64_to_pem(der_base64: &str, label: &str) -> Result<String> {
    let der_bytes = STANDARD.decode(der_base64.trim())?;
    let pem_obj = Pem::new(label, der_bytes);
    let pem = encode(&pem_obj);
    Ok(if pem.ends_with('\n') { pem } else { format!("{}\n", pem) })
}

/// Manual version without the external pem crate (hand-rolled wrapper)
pub fn der_base64_to_pem_manual(der_base64: &str, label: &str) -> Result<String> {
    let der_bytes = STANDARD.decode(der_base64.trim())?;
    let b64 = STANDARD.encode(&der_bytes);

    let wrapped: String = b64.as_bytes()
        .chunks(64)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!("-----BEGIN {label}-----\n{wrapped}\n-----END {label}-----\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let test_key = "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgPfFX/JvA/EmXkxBgDqccfN7F3A7DM4thhwpUhrFt/6ShRANCAAQhxthEiGdGJeZGoGawAkg4XpvbRdm/BOzlE5We0L6Yj+wUVCJ/cvim6UGW+01zyBfOUITgEV7rKhdOlwv2Olgg";
        
        println!("📝 Key length: {}", test_key.len());
        println!("📝 First 30 characters: {}", &test_key[..30]);
        
        let result = der_base64_to_pem(test_key, "PRIVATE KEY");
        
        match result {
            Ok(pem) => {
                println!("✅ Conversion successful!");
                println!("📏 PEM length: {}", pem.len());
                println!("📄 PEM (first 100 chars): {}", &pem[..100]);
                assert!(pem.starts_with("-----BEGIN PRIVATE KEY-----"));
                assert!(pem.contains("-----END PRIVATE KEY-----"));
            }
            Err(e) => {
                println!("❌ Conversion error: {}", e);
                panic!("Conversion failed");
            }
        }
    }
}