use exercise4::crypto_util::{generate_rsa_key_pair, get_private_key, get_public_key};
use exercise4::digital_signature::{sign, verify};
use std::fs;
use tempfile::TempDir;

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .try_init();
}

fn setup_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

fn generate_keys(dir: &TempDir) -> (std::path::PathBuf, std::path::PathBuf) {
    let priv_path = dir.path().join("private.pem");
    let pub_path = dir.path().join("public.pem");
    generate_rsa_key_pair(2048, &priv_path, &pub_path).expect("Key generation failed");
    (priv_path, pub_path)
}

/// Kompletter Workflow: Key generieren → signieren → verifizieren
#[test]
fn test_full_sign_and_verify_workflow() {
    init_tracing();
    let dir = setup_temp_dir();
    let (priv_path, pub_path) = generate_keys(&dir);

    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, b"Hello, digital signature!").unwrap();

    let private_key = get_private_key(&priv_path).expect("Failed to load private key");
    let public_key = get_public_key(&pub_path).expect("Failed to load public key");

    let signature = sign(&file_path, &private_key).expect("Signing failed");

    let sig_path = dir.path().join("data.sig");
    fs::write(&sig_path, &signature).unwrap();

    let valid = verify(&file_path, &sig_path, &public_key).expect("Verification failed");
    assert!(valid, "Signature should be valid");
}

/// Manipulation der Datei muss Verifikation fehlschlagen lassen
#[test]
fn test_tampered_file_fails_verification() {
    init_tracing();
    let dir = setup_temp_dir();
    let (priv_path, pub_path) = generate_keys(&dir);

    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, b"Original content").unwrap();

    let private_key = get_private_key(&priv_path).unwrap();
    let public_key = get_public_key(&pub_path).unwrap();

    let signature = sign(&file_path, &private_key).unwrap();
    let sig_path = dir.path().join("data.sig");
    fs::write(&sig_path, &signature).unwrap();

    // Datei manipulieren
    fs::write(&file_path, b"Tampered content").unwrap();

    let valid = verify(&file_path, &sig_path, &public_key).unwrap();
    assert!(!valid, "Tampered file should fail verification");
}

/// Falscher Public Key muss Verifikation fehlschlagen lassen
#[test]
fn test_wrong_public_key_fails_verification() {
    init_tracing();
    let dir = setup_temp_dir();
    let (priv_path, _) = generate_keys(&dir);

    // Zweites Key-Paar generieren
    let priv2 = dir.path().join("private2.pem");
    let pub2 = dir.path().join("public2.pem");
    generate_rsa_key_pair(2048, &priv2, &pub2).unwrap();

    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, b"Some data").unwrap();

    let private_key = get_private_key(&priv_path).unwrap();
    let wrong_public_key = get_public_key(&pub2).unwrap();

    let signature = sign(&file_path, &private_key).unwrap();
    let sig_path = dir.path().join("data.sig");
    fs::write(&sig_path, &signature).unwrap();

    let valid = verify(&file_path, &sig_path, &wrong_public_key).unwrap();
    assert!(!valid, "Wrong public key should fail verification");
}

/// Leere Datei signieren und verifizieren
#[test]
fn test_sign_and_verify_empty_file() {
    init_tracing();
    let dir = setup_temp_dir();
    let (priv_path, pub_path) = generate_keys(&dir);

    let file_path = dir.path().join("empty.txt");
    fs::write(&file_path, b"").unwrap();

    let private_key = get_private_key(&priv_path).unwrap();
    let public_key = get_public_key(&pub_path).unwrap();

    let signature = sign(&file_path, &private_key).unwrap();
    let sig_path = dir.path().join("empty.sig");
    fs::write(&sig_path, &signature).unwrap();

    let valid = verify(&file_path, &sig_path, &public_key).unwrap();
    assert!(valid, "Empty file signature should be valid");
}

/// Binäre Daten signieren und verifizieren
#[test]
fn test_sign_and_verify_binary_data() {
    init_tracing();
    let dir = setup_temp_dir();
    let (priv_path, pub_path) = generate_keys(&dir);

    let file_path = dir.path().join("binary.bin");
    let binary_data: Vec<u8> = (0u8..=255u8).collect();
    fs::write(&file_path, &binary_data).unwrap();

    let private_key = get_private_key(&priv_path).unwrap();
    let public_key = get_public_key(&pub_path).unwrap();

    let signature = sign(&file_path, &private_key).unwrap();
    let sig_path = dir.path().join("binary.sig");
    fs::write(&sig_path, &signature).unwrap();

    let valid = verify(&file_path, &sig_path, &public_key).unwrap();
    assert!(valid, "Binary file signature should be valid");
}

/// Gespeicherte Keys laden und wiederverwenden
#[test]
fn test_key_persistence_load_and_reuse() {
    init_tracing();
    let dir = setup_temp_dir();
    let priv_path = dir.path().join("private.pem");
    let pub_path = dir.path().join("public.pem");

    generate_rsa_key_pair(2048, &priv_path, &pub_path).unwrap();

    // Keys erneut laden (simuliert neuen Programmlauf)
    let private_key = get_private_key(&priv_path).unwrap();
    let public_key = get_public_key(&pub_path).unwrap();

    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, b"Persisted key test").unwrap();

    let signature = sign(&file_path, &private_key).unwrap();
    let sig_path = dir.path().join("data.sig");
    fs::write(&sig_path, &signature).unwrap();

    let valid = verify(&file_path, &sig_path, &public_key).unwrap();
    assert!(valid, "Keys loaded from disk should work correctly");
}

/// Manipulierte Signatur muss fehlschlagen
#[test]
fn test_tampered_signature_fails_verification() {
    init_tracing();
    let dir = setup_temp_dir();
    let (priv_path, pub_path) = generate_keys(&dir);

    let file_path = dir.path().join("data.txt");
    fs::write(&file_path, b"Important data").unwrap();

    let private_key = get_private_key(&priv_path).unwrap();
    let public_key = get_public_key(&pub_path).unwrap();

    let mut signature = sign(&file_path, &private_key).unwrap();
    // Signatur manipulieren
    if let Some(byte) = signature.get_mut(0) {
        *byte ^= 0xFF;
    }

    let sig_path = dir.path().join("data.sig");
    fs::write(&sig_path, &signature).unwrap();

    let result = verify(&file_path, &sig_path, &public_key);
    // Entweder Fehler oder false — beides ist korrekt
    assert!(
        result.is_err() || !result.unwrap(),
        "Tampered signature should fail"
    );
}
