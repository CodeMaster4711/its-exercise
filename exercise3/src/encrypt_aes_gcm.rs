use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;
use rand::RngCore;
use rand::rngs::OsRng;

const TAG_LENGTH_BIT: usize = 128;
const IV_LENGTH_BYTE: usize = 12; // 96 bit

/// Encrypt data with AES and GCM.
///
/// # Arguments
/// * `plain_text` - text to encrypt
/// * `authentication_data` - additional authentication data used in GCM mode
/// * `key` - symmetric AES key (32 bytes for AES-256)
///
/// # Returns
/// * IV and encrypted data concatenated
pub fn encrypt(plain_text: &[u8], authentication_data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    // TODO: Implement AES-GCM encryption
    // 1. Create cipher with key
    // 2. Generate random IV (12 bytes)
    // 3. Create payload with plaintext and AAD
    // 4. Encrypt
    // 5. Return IV || ciphertext
    
    unimplemented!("TODO: Implement AES-GCM encryption")
}
