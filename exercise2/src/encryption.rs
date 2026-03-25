use anyhow::Result;
use std::path::Path;

const CIPHER_ALGORITHM: &str = "AES/ECB/PKCS5Padding";
const ALGORITHM: &str = "AES";
const KEY_SIZE: usize = 256;

/// Generate a symmetric key with selected algorithm and key size.
///
/// * returns: symmetric SecretKey
pub fn generate_key() -> Result<Vec<u8>> {
    // TODO
    todo!()
}

/// Encrypt data with given key.
///
/// * `key`: SecretKey, generated with generate_key
/// * `data`: byte Array to encrypt
/// * returns: encrypted data
pub fn encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    // TODO
    todo!()
}

/// Save key base64 encoded in a file.
///
/// * `dest_file`: destination filename
/// * `key`: SecretKey to save
pub fn save_key<P: AsRef<Path>>(dest_file: P, key: &[u8]) -> Result<()> {
    // TODO
    todo!()
}
