use anyhow::{Context, Result};
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::sha2::Sha512;
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Sign the contents of a file.
/// Uses SHA-512 as hash-function for computing the digest
/// and RSA for encryption
///
/// # Arguments
/// * `filename` - name of the file to be signed
/// * `signature_key` - private key for signing
///
/// # Returns
/// * signature bytes
pub fn sign<P: AsRef<Path>>(filename: P, signature_key: &RsaPrivateKey) -> Result<Vec<u8>> {
    // TODO: Implement digital signature
    // 1. Read file data
    // 2. Create SHA-512 hash of the data
    // 3. Sign the hash with RSA private key
    // 4. Return signature

    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let signing_key = SigningKey::<Sha512>::new(signature_key.clone());
    let signature = signing_key.sign(&data);

    Ok(signature.to_vec())
}

/// Verify the signature of a signed file.
///
/// # Arguments
/// * `filename` - name of the signed file
/// * `sig_filename` - name of the file where the signature is stored
/// * `pub_key` - public key for verification
///
/// # Returns
/// * true if signature is correct, false if not
pub fn verify<P: AsRef<Path>>(
    filename: P,
    sig_filename: P,
    pub_key: &RsaPublicKey,
) -> Result<bool> {
    // TODO: Implement signature verification
    // 1. Read the file data
    // 2. Create SHA-512 hash of the data
    // 3. Read the signature from sig_filename
    // 4. Verify the signature with RSA public key
    // 5. Return true if valid, false otherwise
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let sig_bytes = std::fs::read(sig_filename)?;
    let verifying_key = VerifyingKey::<Sha512>::new(pub_key.clone());
    let signature = rsa::pkcs1v15::Signature::try_from(sig_bytes.as_slice())
        .map_err(|e| anyhow::anyhow!("Failed to parse signature: {}", e))?;
    match verifying_key.verify(&data, &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
