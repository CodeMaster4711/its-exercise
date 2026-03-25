use aes::cipher::{BlockDecryptMut, KeyInit};
use anyhow::Result;
use std::path::Path;

const CIPHER_ALGORITHM: &str = "AES/ECB/PKCS5Padding";
const ALGORITHM: &str = "AES";

/// Read symmetric key from file and decode it base64.
///
/// * `input_file`: filename
/// * returns: symmetric SecretKey
pub fn read_key<P: AsRef<Path>>(input_file: P) -> Result<Vec<u8>> {
    let encoded_key = std::fs::read_to_string(input_file)?;
    let key = base64::decode(encoded_key)?;
    Ok(key)
}

/// Decrypt data with given key.
///
/// * `key`: SecretKey, read with read_key
/// * `data`: byte Array to decrypt
/// * returns: decrypted data
pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let decrypted = ecb::Decryptor::<aes::Aes256>::new(key.into())
        .decrypt_padded_vec_mut::<block_padding::Pkcs7>(data)
        .map_err(|e| anyhow::anyhow!("Padding error: {:?}", e))?;
    Ok(decrypted)
}
