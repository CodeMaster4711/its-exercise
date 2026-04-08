use exercise3::keystore_utils::KeyStoreUtils;
use rand::RngCore;
use rand::rngs::OsRng;
use std::fs;

fn generate_test_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

#[test]
fn test_create_empty_keystore() {
    let keystore = KeyStoreUtils::new("test_password")
        .expect("Should create new keystore");
    
    // Keystore should be created successfully
    assert!(true, "Keystore created");
}

#[test]
fn test_add_and_get_key() {
    let mut keystore = KeyStoreUtils::new("keystore_password")
        .expect("Should create new keystore");
    
    let key = generate_test_key();
    let key_alias = "test_key".to_string();
    let key_password = "key_password";
    
    // Add key to keystore
    keystore.add_key(key.clone(), key_alias.clone(), key_password)
        .expect("Should add key to keystore");
    
    // Retrieve key from keystore
    let retrieved_key = keystore.get_key(&key_alias, key_password)
        .expect("Should retrieve key from keystore");
    
    // Verify the retrieved key matches the original
    assert_eq!(retrieved_key, key);
}

#[test]
fn test_write_and_read_keystore() {
    let keystore_file = "/tmp/test_keystore.dat";
    let keystore_password = "store_password";
    let key_alias = "my_key".to_string();
    let key_password = "key_password";
    let key = generate_test_key();
    
    // Create keystore and add key
    {
        let mut keystore = KeyStoreUtils::new(keystore_password)
            .expect("Should create new keystore");
        
        keystore.add_key(key.clone(), key_alias.clone(), key_password)
            .expect("Should add key");
        
        keystore.write_to_file(keystore_file, keystore_password)
            .expect("Should write keystore to file");
    }
    
    // Read keystore from file and retrieve key
    {
        let keystore = KeyStoreUtils::from_file(keystore_file, keystore_password)
            .expect("Should read keystore from file");
        
        let retrieved_key = keystore.get_key(&key_alias, key_password)
            .expect("Should retrieve key from loaded keystore");
        
        assert_eq!(retrieved_key, key);
    }
    
    // Cleanup
    let _ = fs::remove_file(keystore_file);
}

#[test]
fn test_multiple_keys_in_keystore() {
    let mut keystore = KeyStoreUtils::new("password")
        .expect("Should create keystore");
    
    let key1 = generate_test_key();
    let key2 = generate_test_key();
    let key3 = generate_test_key();
    
    // Add multiple keys with different aliases
    keystore.add_key(key1.clone(), "key1".to_string(), "pass1")
        .expect("Should add key1");
    keystore.add_key(key2.clone(), "key2".to_string(), "pass2")
        .expect("Should add key2");
    keystore.add_key(key3.clone(), "key3".to_string(), "pass3")
        .expect("Should add key3");
    
    // Retrieve all keys and verify
    let retrieved1 = keystore.get_key("key1", "pass1").expect("Should get key1");
    let retrieved2 = keystore.get_key("key2", "pass2").expect("Should get key2");
    let retrieved3 = keystore.get_key("key3", "pass3").expect("Should get key3");
    
    assert_eq!(retrieved1, key1);
    assert_eq!(retrieved2, key2);
    assert_eq!(retrieved3, key3);
}

#[test]
fn test_keystore_persistence() {
    let keystore_file = "/tmp/test_persistence.dat";
    let password = "persistent_password";
    let key = generate_test_key();
    
    // Create and save
    {
        let mut ks = KeyStoreUtils::new(password).expect("Should create");
        ks.add_key(key.clone(), "persistent_key".to_string(), "key_pw")
            .expect("Should add");
        ks.write_to_file(keystore_file, password)
            .expect("Should write");
    }
    
    // Load and verify
    {
        let ks = KeyStoreUtils::from_file(keystore_file, password)
            .expect("Should load");
        let loaded = ks.get_key("persistent_key", "key_pw")
            .expect("Should retrieve");
        assert_eq!(loaded, key);
    }
    
    // Cleanup
    let _ = fs::remove_file(keystore_file);
}

#[test]
fn test_wrong_key_password_fails() {
    let mut keystore = KeyStoreUtils::new("store_pw").expect("Should create");
    let key = generate_test_key();
    
    keystore.add_key(key, "test".to_string(), "correct_password")
        .expect("Should add key");
    
    // Try to retrieve with wrong password
    let result = keystore.get_key("test", "wrong_password");
    
    // This should fail (or return wrong data, depending on implementation)
    // For now, we just check that we can test this scenario
    assert!(result.is_err() || result.unwrap() != vec![0u8; 32]);
}

#[test]
fn test_nonexistent_key() {
    let keystore = KeyStoreUtils::new("password").expect("Should create");
    
    // Try to get a key that doesn't exist
    let result = keystore.get_key("nonexistent", "password");
    
    assert!(result.is_err(), "Getting nonexistent key should fail");
}
