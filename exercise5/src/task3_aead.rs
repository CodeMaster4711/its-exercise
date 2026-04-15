// Task 3: Symmetrische Verschlüsselung mit AEAD (AES-GCM) via Google Tink
//
// Deine Aufgabe:
//   1. Initialisiere Tink AEAD (tink_aead::init())
//   2. Erzeuge ein KeysetHandle mit AES-GCM-Template
//      Template: tink_aead::aes128_gcm_key_template()  (oder aes256_gcm)
//   3. Hole ein Aead-Primitive aus dem Handle
//   4. Verschlüssele einen Plaintext MIT Associated Data (aad)
//      -> aead.encrypt(plaintext, aad)
//   5. Entschlüssele mit den gleichen aad  -> muss Ok sein
//   6. Entschlüssele mit ANDEREN aad       -> muss Err sein (Integritätsschutz!)

use anyhow::Result;

pub fn task3_aead_demo() -> Result<()> {
    tink_aead::init();

    let kh = tink_core::keyset::Handle::new(&tink_aead::aes128_gcm_key_template())
        .map_err(|e| anyhow::anyhow!("Failed to create keyset handle: {}", e))?;

    let aead =
        tink_aead::new(&kh).map_err(|e| anyhow::anyhow!("Failed to get AEAD primitive: {}", e))?;

    let plaintext = b"AEAD schuetzt Vertraulichkeit und
      Integritaet";
    let aad = b"metadata:exercise5";

    let ciphertext = aead
        .encrypt(plaintext, aad)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
    println!("Ciphertext: {:?}", ciphertext);

    let decrypted = aead
        .decrypt(&ciphertext, aad)
        .map_err(|e| anyhow::anyhow!("Decryption with correct AAD failed: {}", e))?;

    println!("Plaintext: {}", String::from_utf8_lossy(&decrypted));
    assert_eq!(plaintext.as_ref(), decrypted.as_slice());

    assert!(aead.decrypt(&ciphertext, b"falsches-label").is_err());
    println!(
        "Entschlüsseln mit falschen AAD: Err
            (korrekt)"
    );

    let mut tampered = ciphertext.clone();
    let last = tampered.len() - 1;
    tampered[last] ^= 0xFF;
    assert!(aead.decrypt(&tampered, aad).is_err());
    println!("Manipulierter Ciphertext: Err (korrekt)");

    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
