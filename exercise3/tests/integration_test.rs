use exercise3::{encrypt_aes_gcm, decrypt_aes_gcm, keystore_utils::KeyStoreUtils, stream_encryption, stream_decryption};
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs;
use std::io::Write;

fn generate_test_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

#[test]
fn test_complete_workflow_with_keystore() {
    // Setup
    let keystore_file = "/tmp/integration_keystore.dat";
    let keystore_password = "store_password";
    let key_alias = "encryption_key".to_string();
    let key_password = "key_password";
    let plaintext = b"This is a complete workflow test with keystore";
    let aad = b"Additional Authentication Data";
    
    // Step 1: Generate key and store in keystore
    let key = generate_test_key();
    {
        let mut keystore = KeyStoreUtils::new(keystore_password)
            .expect("Should create keystore");
        
        keystore.add_key(key.clone(), key_alias.clone(), key_password)
            .expect("Should add key");
        
        keystore.write_to_file(keystore_file, keystore_password)
            .expect("Should write keystore");
    }
    
    // Step 2: Load keystore and retrieve key
    let loaded_key = {
        let keystore = KeyStoreUtils::from_file(keystore_file, keystore_password)
            .expect("Should load keystore");
        
        keystore.get_key(&key_alias, key_password)
            .expect("Should retrieve key")
    };
    
    // Step 3: Encrypt with loaded key
    let ciphertext = encrypt_aes_gcm::encrypt(plaintext, aad, &loaded_key)
        .expect("Encryption should succeed");
    
    // Step 4: Decrypt with loaded key
    let decrypted = decrypt_aes_gcm::decrypt(&ciphertext, aad, &loaded_key)
        .expect("Decryption should succeed");
    
    // Verify
    assert_eq!(decrypted, plaintext);
    
    // Cleanup
    let _ = fs::remove_file(keystore_file);
}

#[test]
fn test_stream_encryption_with_keystore() {
    // Setup
    let keystore_file = "/tmp/stream_keystore.dat";
    let input_file = "/tmp/stream_test_input.txt";
    let encrypted_file = "/tmp/stream_test_encrypted.bin";
    let decrypted_file = "/tmp/stream_test_decrypted.txt";
    let keystore_password = "store_pw";
    let key_alias = "stream_key".to_string();
    let key_password = "key_pw";
    
    // Create test content
    let test_content = b"Stream encryption test with keystore integration.\nMultiple lines of text.\nTo test stream processing.";
    let mut file = fs::File::create(input_file).expect("Should create file");
    file.write_all(test_content).expect("Should write");
    drop(file);
    
    // Generate and store key
    let key = generate_test_key();
    {
        let mut ks = KeyStoreUtils::new(keystore_password).expect("Should create");
        ks.add_key(key, key_alias.clone(), key_password).expect("Should add");
        ks.write_to_file(keystore_file, keystore_password).expect("Should write");
    }
    
    // Load key from keystore
    let loaded_key = {
        let ks = KeyStoreUtils::from_file(keystore_file, keystore_password)
            .expect("Should load");
        ks.get_key(&key_alias, key_password).expect("Should get")
    };
    
    // Encrypt file
    stream_encryption::encrypt(input_file, encrypted_file, &loaded_key)
        .expect("Should encrypt");
    
    // Decrypt file
    stream_decryption::decrypt(encrypted_file, decrypted_file, &loaded_key)
        .expect("Should decrypt");
    
    // Verify
    let decrypted_content = fs::read(decrypted_file).expect("Should read");
    assert_eq!(decrypted_content, test_content);
    
    // Cleanup
    let _ = fs::remove_file(keystore_file);
    let _ = fs::remove_file(input_file);
    let _ = fs::remove_file(encrypted_file);
    let _ = fs::remove_file(decrypted_file);
    let _ = fs::remove_file("iv.bin");
}

#[test]
fn test_multiple_files_same_keystore() {
    let keystore_file = "/tmp/multi_keystore.dat";
    let keystore_pw = "store_pw";
    
    // Create keystore with multiple keys
    let key1 = generate_test_key();
    let key2 = generate_test_key();
    let key3 = generate_test_key();
    
    {
        let mut ks = KeyStoreUtils::new(keystore_pw).expect("Should create");
        ks.add_key(key1.clone(), "key1".to_string(), "pw1").expect("Should add");
        ks.add_key(key2.clone(), "key2".to_string(), "pw2").expect("Should add");
        ks.add_key(key3.clone(), "key3".to_string(), "pw3").expect("Should add");
        ks.write_to_file(keystore_file, keystore_pw).expect("Should write");
    }
    
    // Encrypt data with different keys
    let data1 = b"Data for key 1";
    let data2 = b"Data for key 2";
    let data3 = b"Data for key 3";
    let aad = b"";
    
    let ct1 = encrypt_aes_gcm::encrypt(data1, aad, &key1).expect("Should encrypt");
    let ct2 = encrypt_aes_gcm::encrypt(data2, aad, &key2).expect("Should encrypt");
    let ct3 = encrypt_aes_gcm::encrypt(data3, aad, &key3).expect("Should encrypt");
    
    // Load keystore and decrypt with each key
    let ks = KeyStoreUtils::from_file(keystore_file, keystore_pw).expect("Should load");
    
    let loaded_key1 = ks.get_key("key1", "pw1").expect("Should get");
    let loaded_key2 = ks.get_key("key2", "pw2").expect("Should get");
    let loaded_key3 = ks.get_key("key3", "pw3").expect("Should get");
    
    let dec1 = decrypt_aes_gcm::decrypt(&ct1, aad, &loaded_key1).expect("Should decrypt");
    let dec2 = decrypt_aes_gcm::decrypt(&ct2, aad, &loaded_key2).expect("Should decrypt");
    let dec3 = decrypt_aes_gcm::decrypt(&ct3, aad, &loaded_key3).expect("Should decrypt");
    
    assert_eq!(dec1, data1);
    assert_eq!(dec2, data2);
    assert_eq!(dec3, data3);
    
    // Cleanup
    let _ = fs::remove_file(keystore_file);
}

#[test]
fn test_aes_gcm_with_different_aad() {
    let key = generate_test_key();
    let plaintext = b"Test message";
    let aad1 = b"AAD version 1";
    let aad2 = b"AAD version 2";
    
    // Encrypt with AAD1
    let ct1 = encrypt_aes_gcm::encrypt(plaintext, aad1, &key)
        .expect("Should encrypt");
    
    // Encrypt with AAD2
    let ct2 = encrypt_aes_gcm::encrypt(plaintext, aad2, &key)
        .expect("Should encrypt");
    
    // Both should decrypt with their respective AAD
    let dec1 = decrypt_aes_gcm::decrypt(&ct1, aad1, &key)
        .expect("Should decrypt with AAD1");
    let dec2 = decrypt_aes_gcm::decrypt(&ct2, aad2, &key)
        .expect("Should decrypt with AAD2");
    
    assert_eq!(dec1, plaintext);
    assert_eq!(dec2, plaintext);
    
    // Cross-decryption should fail
    let result1 = decrypt_aes_gcm::decrypt(&ct1, aad2, &key);
    let result2 = decrypt_aes_gcm::decrypt(&ct2, aad1, &key);
    
    assert!(result1.is_err(), "Wrong AAD should fail");
    assert!(result2.is_err(), "Wrong AAD should fail");
}

#[test]
fn test_keystore_key_overwrite() {
    let mut ks = KeyStoreUtils::new("password").expect("Should create");
    
    let key1 = generate_test_key();
    let key2 = generate_test_key();
    let alias = "same_alias".to_string();
    
    // Add first key
    ks.add_key(key1.clone(), alias.clone(), "pw").expect("Should add");
    
    // Add second key with same alias (should overwrite)
    ks.add_key(key2.clone(), alias.clone(), "pw").expect("Should add");
    
    // Retrieved key should be the second one
    let retrieved = ks.get_key(&alias, "pw").expect("Should get");
    assert_eq!(retrieved, key2);
    assert_ne!(retrieved, key1);
}
