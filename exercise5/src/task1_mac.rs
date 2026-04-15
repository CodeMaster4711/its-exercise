// Task 1: Berechnung eines MAC mit Google Tink (HMAC-SHA256)

use anyhow::{anyhow, Result};

pub fn task1_mac_demo() -> Result<()> {
    tink_mac::init();

    let kh = tink_core::keyset::Handle::new(&tink_mac::hmac_sha256_tag256_key_template())
        .map_err(|e| anyhow!(e.to_string()))?;

    let mac = tink_mac::new(&kh).map_err(|e| anyhow!(e.to_string()))?;

    let data = b"Meine Nachricht";

    // MAC berechnen
    let tag = mac.compute_mac(data).map_err(|e| anyhow!(e.to_string()))?;
    println!("Tag (hex): {}", hex(&tag));

    // Verifizieren: korrekte Nachricht -> Ok
    mac.verify_mac(&tag, data).map_err(|e| anyhow!(e.to_string()))?;
    println!("Verifikation korrekt: Ok");

    // Verifizieren: falsche Nachricht -> Err
    match mac.verify_mac(&tag, b"Manipulierte Nachricht") {
        Ok(_) => println!("Verifikation falsch: sollte nicht Ok sein!"),
        Err(_) => println!("Verifikation manipuliert: Err (korrekt)"),
    }

    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
