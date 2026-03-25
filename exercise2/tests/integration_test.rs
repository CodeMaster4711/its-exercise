use exercise2::encryption;
use exercise2::decryption;

#[test]
fn key_has_correct_length() {
    let key = encryption::generate_key().unwrap();
    assert_eq!(key.len(), 32); // 256 bit = 32 Bytes
}

#[test]
fn encrypted_data_is_not_plaintext() {
    let key = encryption::generate_key().unwrap();
    let plaintext = b"Hello, World!";
    let ciphertext = encryption::encrypt(&key, plaintext).unwrap();
    assert_ne!(ciphertext, plaintext);
}

#[test]
fn encrypted_length_is_plaintext_plus_overhead() {
    let key = encryption::generate_key().unwrap();
    let plaintext = b"Hello, World!";
    let ciphertext = encryption::encrypt(&key, plaintext).unwrap();
    // GCM: 12 Bytes Nonce + Plaintext + 16 Bytes Auth-Tag
    assert_eq!(ciphertext.len(), plaintext.len() + 12 + 16);
}

#[test]
fn encrypt_then_decrypt_returns_original() {
    let key = encryption::generate_key().unwrap();
    let plaintext = b"Hello, World!";
    let ciphertext = encryption::encrypt(&key, plaintext).unwrap();
    let decrypted = decryption::decrypt(&key, &ciphertext).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn save_and_read_key_roundtrip() {
    let key = encryption::generate_key().unwrap();
    let path = "/tmp/test_key.txt";
    encryption::save_key(path, &key).unwrap();
    let loaded_key = decryption::read_key(path).unwrap();
    assert_eq!(key, loaded_key);
}

#[test]
fn different_keys_produce_different_ciphertext() {
    let key1 = encryption::generate_key().unwrap();
    let key2 = encryption::generate_key().unwrap();
    let plaintext = b"Hello, World!";
    let ct1 = encryption::encrypt(&key1, plaintext).unwrap();
    let ct2 = encryption::encrypt(&key2, plaintext).unwrap();
    assert_ne!(ct1, ct2);
}
