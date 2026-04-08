use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const IV_FILE: &str = "iv.bin";
const TAG_LENGTH_BIT: usize = 128;
const IV_LENGTH_BYTE: usize = 12; // 96 bit

/// Encrypt the data.
/// Data is read, encrypted and encoded Base64 by streams.
///
/// # Arguments
/// * `input_file` - file data
/// * `enc_file` - file with encrypted data
/// * `key` - symmetric AES key
pub fn encrypt<P: AsRef<Path>>(input_file: P, enc_file: P, key: &[u8]) -> Result<()> {
    // TODO: Implement stream encryption
    // 1. Read input file
    // 2. Generate random IV
    // 3. Save IV to IV_FILE
    // 4. Encrypt data using AES-GCM
    // 5. Encode encrypted data as Base64
    // 6. Write to output file
    
    unimplemented!("TODO: Implement stream encryption")
}
