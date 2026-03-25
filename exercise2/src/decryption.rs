use anyhow::Result;
use std::path::Path;

const CIPHER_ALGORITHM: &str = "AES/ECB/PKCS5Padding";
const ALGORITHM: &str = "AES";

/// Read symmetric key from file and decode it base64.
///
/// * `input_file`: filename
/// * returns: symmetric SecretKey
pub fn read_key<P: AsRef<Path>>(input_file: P) -> Result<Vec<u8>> {
    // TODO
    todo!()
}

/// Decrypt data with given key.
///
/// * `key`: SecretKey, read with read_key
/// * `data`: byte Array to decrypt
/// * returns: decrypted data
pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    // TODO
    todo!()
}

