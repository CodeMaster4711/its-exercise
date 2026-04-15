// Task 4: Digitale Signatur mit Google Tink (ECDSA)

use anyhow::{anyhow, Result};

pub fn task4_signature_demo() -> Result<()> {
    tink_signature::init();

    // 1. Privaten KeysetHandle erzeugen
    let private_kh = tink_core::keyset::Handle::new(&tink_signature::ecdsa_p256_key_template())
        .map_err(|e| anyhow!(e.to_string()))?;

    // 2. Öffentlichen Handle ableiten
    let public_kh = private_kh.public()
        .map_err(|e| anyhow!(e.to_string()))?;

    // 3. Primitives holen
    let signer = tink_signature::new_signer(&private_kh)
        .map_err(|e| anyhow!(e.to_string()))?;
    let verifier = tink_signature::new_verifier(&public_kh)
        .map_err(|e| anyhow!(e.to_string()))?;

    let message = b"Digitale Signatur Demo";

    // 4. Signieren mit private key
    let signature = signer.sign(message)
        .map_err(|e| anyhow!(e.to_string()))?;
    println!("Signatur (hex): {}", hex(&signature));

    // 5. Verifizieren mit public key + korrekte Nachricht -> Ok
    verifier.verify(&signature, message)
        .map_err(|e| anyhow!(e.to_string()))?;
    println!("Signatur gueltig: Ok");

    // 6. Andere Nachricht -> Err
    assert!(verifier.verify(&signature, b"Veraenderte Nachricht").is_err());
    println!("Signatur bei veraenderter Nachricht: Err (korrekt)");

    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
