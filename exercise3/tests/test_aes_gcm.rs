use exercise3::{encrypt_aes_gcm, decrypt_aes_gcm};
use rand::RngCore;
use rand::rngs::OsRng;

fn generate_test_key() -> Vec<u8> {
    let mut key = vec![0u8; 32]; // 256 bit for AES-256
    OsRng.fill_bytes(&mut key);
    key
}

#[test]
fn test_encrypt_decrypt_without_aad() {
    let key = generate_test_key();
    let plaintext = b"Hello, World! This is a test message.";
    let aad = b""; // No additional authentication data
    
    // Encrypt
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Encryption should succeed");
    
    // Verify ciphertext is different from plaintext
    assert_ne!(&ciphertext[12..], plaintext);
    
    // Decrypt
    let decrypted = decrypt_aes_gcm::decrypt(&ciphertext, aad, &key)
        .expect("Decryption should succeed");
    
    // Verify decrypted matches original
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_encrypt_decrypt_with_aad() {
    let key = generate_test_key();
    let plaintext = b"Secret message with authentication";
    let aad = b"Any Authentication Data";
    
    // Encrypt with AAD
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Encryption should succeed");
    
    // Decrypt with same AAD
    let decrypted = decrypt_aes_gcm::decrypt(&ciphertext, aad, &key)
        .expect("Decryption should succeed");
    
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_wrong_aad_fails() {
    let key = generate_test_key();
    let plaintext = b"Secret message";
    let aad = b"Correct AAD";
    let wrong_aad = b"Wrong AAD";
    
    // Encrypt with correct AAD
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Encryption should succeed");
    
    // Try to decrypt with wrong AAD - should fail
    let result = decrypt_aes_gcm::decrypt(&ciphertext, wrong_aad, &key);
    assert!(result.is_err(), "Decryption with wrong AAD should fail");
}

#[test]
fn test_wrong_key_fails() {
    let key1 = generate_test_key();
    let key2 = generate_test_key();
    let plaintext = b"Secret message";
    let aad = b"Test AAD";
    
    // Encrypt with key1
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key1)
        .expect("Encryption should succeed");
    
    // Try to decrypt with key2 - should fail
    let result = decrypt_aes_gcm::decrypt(&ciphertext, aad, &key2);
    assert!(result.is_err(), "Decryption with wrong key should fail");
}

#[test]
fn test_ciphertext_manipulation_detected() {
    let key = generate_test_key();
    let plaintext = b"Important data that must not be manipulated";
    let aad = b"Integrity check";
    
    // Encrypt
    let mut ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Encryption should succeed");
    
    // Manipulate ciphertext (flip some bits)
    if ciphertext.len() > 20 {
        ciphertext[15] ^= 0xFF; // Flip bits in IV or ciphertext
        ciphertext[20] ^= 0xFF;
    }
    
    // Try to decrypt manipulated data - should fail
    let result = decrypt_aes_gcm::decrypt(&ciphertext, aad, &key);
    assert!(result.is_err(), "Decryption of manipulated data should fail");
}

#[test]
fn test_ciphertext_has_correct_length() {
    let key = generate_test_key();
    let plaintext = b"Test message";
    let aad = b"AAD";
    
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Encryption should succeed");
    
    // GCM: 12 bytes IV + plaintext length + 16 bytes auth tag
    assert_eq!(ciphertext.len(), 12 + plaintext.len() + 16);
}

#[test]
fn test_empty_message() {
    let key = generate_test_key();
    let plaintext = b"";
    let aad = b"Only AAD, no data";
    
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Encryption should succeed");
    
    let decrypted = decrypt_aes_gcm::decrypt(&ciphertext, aad, &key)
        .expect("Decryption should succeed");
    
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_different_encryptions_produce_different_ciphertext() {
    let key = generate_test_key();
    let plaintext = b"Same message encrypted twice";
    let aad = b"Same AAD";
    
    let ct1 = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("First encryption should succeed");
    let ct2 = encrypt_aes_gcm::encrypt(plaintext, aad, &key)
        .expect("Second encryption should succeed");
    
    // Due to random IV, ciphertexts should be different
    assert_ne!(ct1, ct2);
    
    // But both should decrypt to the same plaintext
    let dec1 = decrypt_aes_gcm::decrypt(&ct1, aad, &key).unwrap();
    let dec2 = decrypt_aes_gcm::decrypt(&ct2, aad, &key).unwrap();
    assert_eq!(dec1, plaintext);
    assert_eq!(dec2, plaintext);
}
