use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use rand::RngCore;
use rand::rngs::OsRng;
use std::path::Path;

/// Generate a symmetric key with selected algorithm and key size.
///
/// * returns: symmetric SecretKey
pub fn generate_key() -> Result<Vec<u8>> {
    let mut key = vec![0u8; 32];
    OsRng.fill_bytes(&mut key);
    Ok(key)
}

/// Encrypt data with given key.
///
/// * `key`: SecretKey, generated with generate_key
/// * `data`: byte Array to encrypt
/// * returns: encrypted data
pub fn encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(key.into());
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption error: {:?}", e))?;

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Save key base64 encoded in a file.
///
/// * `dest_file`: destination filename
/// * `key`: SecretKey to save
pub fn save_key<P: AsRef<Path>>(dest_file: P, key: &[u8]) -> Result<()> {
    let encoded_key = STANDARD.encode(key);
    std::fs::write(dest_file, encoded_key)?;
    Ok(())
}
