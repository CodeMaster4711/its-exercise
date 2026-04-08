use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const IV_FILE: &str = "iv.bin";
const TAG_LENGTH_BIT: usize = 128;
const IV_LENGTH_BYTE: usize = 12; // 96 bit

/// Decrypt the data.
/// Data is read, decoded Base64 and decrypted by streams.
///
/// # Arguments
/// * `input_file_name` - encrypted filename
/// * `output_file_name` - decrypted filename
/// * `key` - symmetric AES key
pub fn decrypt<P: AsRef<Path>>(input_file_name: P, output_file_name: P, key: &[u8]) -> Result<()> {
    // TODO: Implement stream decryption
    // 1. Read IV from IV_FILE
    // 2. Read encrypted data from input file
    // 3. Decode Base64
    // 4. Decrypt using AES-GCM
    // 5. Write plaintext to output file
    
    unimplemented!("TODO: Implement stream decryption")
}
