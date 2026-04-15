// Task 2: Hybrid Encryption mit Google Tink (ECIES-AEAD-HKDF + AES-GCM)
//
// Deine Aufgabe:
//   1. Initialisiere Tink Hybrid (tink_hybrid::init())
//   2. Erzeuge ein KeysetHandle für den PRIVATEN Schlüssel
//      Template: tink_hybrid::ecies_hkdf_hmac_sha256_aes128_gcm_key_template()
//   3. Leite daraus den PUBLIC KeysetHandle ab (handle.public()?)
//   4. Verschlüssele Daten mit dem HybridEncrypt-Primitive (public key)
//      -> context_info kann ein leerer &[] oder ein Label sein
//   5. Entschlüssele mit dem HybridDecrypt-Primitive (private key)
//   6. Stelle sicher, dass Plaintext == entschlüsselter Text

use anyhow::Result;

pub fn task2_hybrid_demo() -> Result<()> {
    tink_hybrid::init();

    let private_kh = tink_core::keyset::Handle::new(
        &tink_hybrid::ecies_hkdf_aes128_gcm_key_template(),
    )
    .map_err(|e| anyhow::anyhow!("Failed to create private key handle: {}", e))?;

    let public_kh = private_kh
        .public()
        .map_err(|e| anyhow::anyhow!("Failed to derive public key handle: {}", e))?;

    let encryptor = tink_hybrid::new_encrypt(&public_kh)
        .map_err(|e| anyhow::anyhow!("Failed to create HybridEncrypt primitive: {}", e))?;
    let decryptor = tink_hybrid::new_decrypt(&private_kh)
        .map_err(|e| anyhow::anyhow!("Failed to create HybridDecrypt primitive: {}", e))?;

    let plaintext = b"Hello, Hybrid Encryption!";
    let context_info = b"example context info";

    let ciphertext = encryptor
        .encrypt(plaintext, context_info)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    println!("Ciphertext bytes: {}", ciphertext.len());

    let decrypted = decryptor
        .decrypt(&ciphertext, context_info)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
    println!(
        "Decrypted plaintext: {}",
        String::from_utf8_lossy(&decrypted)
    );

    assert!(decryptor.decrypt(&ciphertext, b"falscher-kontext").is_err());
    println!(
        "Entschlüsseln mit falschem context: Err
      (korrekt)"
    );

    Ok(())
}
