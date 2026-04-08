use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;

const TAG_LENGTH_BIT: usize = 128;
const IV_LENGTH_BYTE: usize = 12; // 96 bit

/// Decrypt text with AES and GCM
///
/// # Arguments
/// * `cipher_text` - IV and ciphertext concatenated
/// * `authentication_data` - additional authentication data used in GCM mode
/// * `key` - symmetric AES key (32 bytes for AES-256)
///
/// # Returns
/// * plaintext
pub fn decrypt(cipher_text: &[u8], authentication_data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // TODO: Implement AES-GCM decryption
    // 1. Extract IV (first 12 bytes)
    // 2. Extract ciphertext (remaining bytes)
    // 3. Create cipher with key
    // 4. Create payload with ciphertext and AAD
    // 5. Decrypt
    // 6. Return plaintext
    
    unimplemented!("TODO: Implement AES-GCM decryption")
}
