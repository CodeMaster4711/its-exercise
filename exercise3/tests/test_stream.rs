use exercise3::{stream_encryption, stream_decryption};
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs;
use std::io::Write;

fn generate_test_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn create_test_file(path: &str, content: &[u8]) {
    let mut file = fs::File::create(path).expect("Should create test file");
    file.write_all(content).expect("Should write test content");
}

#[test]
fn test_stream_encrypt_decrypt_small_file() {
    let key = generate_test_key();
    let input_file = "/tmp/test_input.txt";
    let encrypted_file = "/tmp/test_encrypted.bin";
    let decrypted_file = "/tmp/test_decrypted.txt";
    let test_content = b"Hello, World! This is a test file for stream encryption.";
    
    // Create test file
    create_test_file(input_file, test_content);
    
    // Encrypt
    stream_encryption::encrypt(input_file, encrypted_file, &key)
        .expect("Encryption should succeed");
    
    // Verify encrypted file exists and is different
    let encrypted_content = fs::read(encrypted_file).expect("Should read encrypted file");
    assert_ne!(encrypted_content, test_content);
    
    // Decrypt
    stream_decryption::decrypt(encrypted_file, decrypted_file, &key)
        .expect("Decryption should succeed");
    
    // Verify decrypted content matches original
    let decrypted_content = fs::read(decrypted_file).expect("Should read decrypted file");
    assert_eq!(decrypted_content, test_content);
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_stream_encrypt_decrypt_large_file() {
    let key = generate_test_key();
    let input_file = "/tmp/test_large_input.txt";
    let encrypted_file = "/tmp/test_large_encrypted.bin";
    let decrypted_file = "/tmp/test_large_decrypted.txt";
    
    // Create a larger test file (10KB)
    let mut large_content = Vec::new();
    for i in 0..1000 {
        large_content.extend_from_slice(format!("Line number {}\n", i).as_bytes());
    }
    create_test_file(input_file, &large_content);
    
    // Encrypt
    stream_encryption::encrypt(input_file, encrypted_file, &key)
        .expect("Encryption should succeed");
    
    // Decrypt
    stream_decryption::decrypt(encrypted_file, decrypted_file, &key)
        .expect("Decryption should succeed");
    
    // Verify
    let decrypted_content = fs::read(decrypted_file).expect("Should read decrypted file");
    assert_eq!(decrypted_content, large_content);
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_stream_empty_file() {
    let key = generate_test_key();
    let input_file = "/tmp/test_empty.txt";
    let encrypted_file = "/tmp/test_empty_encrypted.bin";
    let decrypted_file = "/tmp/test_empty_decrypted.txt";
    
    // Create empty file
    create_test_file(input_file, b"");
    
    // Encrypt
    stream_encryption::encrypt(input_file, encrypted_file, &key)
        .expect("Encryption of empty file should succeed");
    
    // Decrypt
    stream_decryption::decrypt(encrypted_file, decrypted_file, &key)
        .expect("Decryption should succeed");
    
    // Verify
    let decrypted_content = fs::read(decrypted_file).expect("Should read file");
    assert_eq!(decrypted_content.len(), 0);
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_stream_binary_data() {
    let key = generate_test_key();
    let input_file = "/tmp/test_binary.bin";
    let encrypted_file = "/tmp/test_binary_encrypted.bin";
    let decrypted_file = "/tmp/test_binary_decrypted.bin";
    
    // Create file with binary data
    let binary_content: Vec<u8> = (0..256).map(|i| i as u8).collect();
    create_test_file(input_file, &binary_content);
    
    // Encrypt
    stream_encryption::encrypt(input_file, encrypted_file, &key)
        .expect("Encryption should succeed");
    
    // Decrypt
    stream_decryption::decrypt(encrypted_file, decrypted_file, &key)
        .expect("Decryption should succeed");
    
    // Verify
    let decrypted_content = fs::read(decrypted_file).expect("Should read file");
    assert_eq!(decrypted_content, binary_content);
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_stream_wrong_key_fails() {
    let key1 = generate_test_key();
    let key2 = generate_test_key();
    let input_file = "/tmp/test_wrong_key.txt";
    let encrypted_file = "/tmp/test_wrong_key_encrypted.bin";
    let decrypted_file = "/tmp/test_wrong_key_decrypted.txt";
    
    create_test_file(input_file, b"Secret content");
    
    // Encrypt with key1
    stream_encryption::encrypt(input_file, encrypted_file, &key1)
        .expect("Encryption should succeed");
    
    // Try to decrypt with key2
    let result = stream_decryption::decrypt(encrypted_file, decrypted_file, &key2);
    
    assert!(result.is_err(), "Decryption with wrong key should fail");
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_stream_base64_encoding() {
    let key = generate_test_key();
    let input_file = "/tmp/test_base64.txt";
    let encrypted_file = "/tmp/test_base64_encrypted.txt";
    let decrypted_file = "/tmp/test_base64_decrypted.txt";
    
    create_test_file(input_file, b"Testing Base64 encoding");
    
    // Encrypt (should produce Base64 encoded output)
    stream_encryption::encrypt(input_file, encrypted_file, &key)
        .expect("Encryption should succeed");
    
    // Read encrypted file and verify it's Base64 (all ASCII printable)
    let encrypted_content = fs::read_to_string(encrypted_file)
        .expect("Should read as string");
    
    // Base64 should only contain A-Z, a-z, 0-9, +, /, =
    assert!(encrypted_content.chars().all(|c| 
        c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' || c.is_whitespace()
    ));
    
    // Decrypt
    stream_decryption::decrypt(encrypted_file, decrypted_file, &key)
        .expect("Decryption should succeed");
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_iv_file_created() {
    let key = generate_test_key();
    let input_file = "/tmp/test_iv.txt";
    let encrypted_file = "/tmp/test_iv_encrypted.bin";
    let iv_file = "iv.bin";
    
    // Remove IV file if it exists
    let _ = fs::remove_file(iv_file);
    
    create_test_file(input_file, b"Test IV creation");
    
    // Encrypt
    stream_encryption::encrypt(input_file, encrypted_file, &key)
        .expect("Encryption should succeed");
    
    // Verify IV file was created
    assert!(fs::metadata(iv_file).is_ok(), "IV file should exist");
    
    // Verify IV has correct length (12 bytes for GCM)
    let iv_content = fs::read(iv_file).expect("Should read IV file");
    assert_eq!(iv_content.len(), 12, "IV should be 12 bytes");
    
    // Cleanup
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(iv_file);
}
