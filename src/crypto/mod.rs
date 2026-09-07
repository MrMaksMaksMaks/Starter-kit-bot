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
        // Not a cryptographic key — arbitrary bytes used only to verify that
        // der_base64_to_pem correctly base64-decodes its input and wraps the
        // result in PEM armor. Cryptographic validity of a real key is
        // exercised separately, at actual use time, via
        // EncodingKey::from_ec_pem in openfort/mod.rs — not here. Deliberately
        // NOT a key-shaped value, so it can never be mistaken for (or
        // flagged by secret scanners as) real key material.
        let dummy_bytes = b"not-a-real-key-just-arbitrary-test-bytes-for-pem-wrapping";
        let test_key = STANDARD.encode(dummy_bytes);

        println!("📝 Test input length: {}", test_key.len());
        println!("📝 First 30 characters: {}", &test_key[..30]);

        let result = der_base64_to_pem(&test_key, "PRIVATE KEY");

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
                panic!("Conversion failed: {}", e);
            }
        }
    }
}
