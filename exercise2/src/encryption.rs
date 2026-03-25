use aes::Aes256;
use aes::cipher::{BlockEncryptMut, KeyInit};
use anyhow::Result;
use block_padding::Pkcs7;
use ecb::Encryptor;
use rand::RngCore;
use rand::rngs::OsRng;
use std::path::Path;

const CIPHER_ALGORITHM: &str = "AES/ECB/PKCS5Padding";
const ALGORITHM: &str = "AES";
const KEY_SIZE: usize = 256;

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
    let encrypted = Encryptor::<Aes256>::new(key.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    Ok(encrypted)
}

/// Save key base64 encoded in a file.
///
/// * `dest_file`: destination filename
/// * `key`: SecretKey to save
pub fn save_key<P: AsRef<Path>>(dest_file: P, key: &[u8]) -> Result<()> {
    let encoded_key = base64::encode(key);
    std::fs::write(dest_file, encoded_key)?;
    Ok(())
}
