use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use std::path::Path;

/// Read symmetric key from file and decode it base64.
///
/// * `input_file`: filename
/// * returns: symmetric SecretKey
pub fn read_key<P: AsRef<Path>>(input_file: P) -> Result<Vec<u8>> {
    let encoded_key = std::fs::read_to_string(input_file)?;
    let key = STANDARD.decode(encoded_key.trim())?;
    Ok(key)
}

/// Decrypt data with given key.
///
/// * `key`: SecretKey, read with read_key
/// * `data`: byte Array to decrypt
/// * returns: decrypted data
pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new(key.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decrypt error: {:?}", e))?;
    Ok(plaintext)
}
