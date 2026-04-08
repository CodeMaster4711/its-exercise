use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use rand::rngs::OsRng;
use rsa::pkcs8::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
    PrivateKeyInfo,
};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs;
use std::path::Path;

/// Read Base64 encoded public key from file in format PEM
///
/// # Arguments
/// * `file` - name of file containing public key
///
/// # Returns
/// * public key
pub fn get_public_key<P: AsRef<Path>>(file: P) -> Result<RsaPublicKey> {
    // TODO: Implement reading public key from PEM file
    // 1. Read the PEM file
    // 2. Parse the public key using rsa::pkcs8::DecodePublicKey
    // 3. Return the public key
    let pem = fs::read_to_string(file).context("Failed to read public key file")?;
    let key =
        RsaPublicKey::from_public_key_pem(&pem).context("Failed to parse public key from PEM")?;
    Ok(key)
}

/// Read Base64 encoded private key from file in format PEM
///
/// # Arguments
/// * `filename` - filename of private key file
///
/// # Returns
/// * private key
pub fn get_private_key<P: AsRef<Path>>(filename: P) -> Result<RsaPrivateKey> {
    // TODO: Implement reading private key from PEM file
    // 1. Read the PEM file
    // 2. Parse the private key using rsa::pkcs8::DecodePrivateKey
    // 3. Return the private key
    let pem = fs::read_to_string(filename).context("Failed to read private key file")?;
    let key =
        RsaPrivateKey::from_pkcs8_pem(&pem).context("Failed to parse private key from PEM")?;
    Ok(key)
}

/// Generate a key pair for RSA and store the two keys in files
///
/// # Arguments
/// * `length` - key length of RSA keys (e.g., 2048, 4096)
/// * `private_key_file` - filename for private key
/// * `public_key_file` - filename for public key
pub fn generate_rsa_key_pair<P: AsRef<Path>>(
    length: usize,
    private_key_file: P,
    public_key_file: P,
) -> Result<()> {
    // TODO: Implement RSA key pair generation
    // 1. Generate RSA key pair with specified bit length
    // 2. Save private key to file in PEM format (PKCS#8)
    // 3. Save public key to file in PEM format
    let mut rng = OsRng;
    let private_key =
        RsaPrivateKey::new(&mut rng, length).context("Failed to generate RSA private key")?;
    let public_key = RsaPublicKey::from(&private_key);

    let private_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .context("Failed to encode private key to PEM")?;
    let public_pem = public_key
        .to_public_key_pem(LineEnding::LF)
        .context("Failed to encode public key to PEM")?;

    Ok(())
}
