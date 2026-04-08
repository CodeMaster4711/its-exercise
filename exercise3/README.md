# Exercise 3: AES-GCM Encryption and KeyStore

This exercise implements AES encryption with GCM (Galois/Counter Mode) and a simple KeyStore utility in Rust.

## Modules

- **encrypt_aes_gcm.rs**: Encrypt data with AES-GCM
- **decrypt_aes_gcm.rs**: Decrypt data with AES-GCM
- **keystore_utils.rs**: Simple KeyStore for managing symmetric keys
- **stream_encryption.rs**: Stream-based encryption with Base64 encoding
- **stream_decryption.rs**: Stream-based decryption with Base64 decoding

## Dependencies

- `aes-gcm`: AES-GCM encryption
- `base64`: Base64 encoding/decoding
- `rand`: Random number generation
- `anyhow`: Error handling

## Build

```bash
cargo build -p exercise3
```

## Run

```bash
cargo run -p exercise3
```

## Tests

Comprehensive test suite covering all modules:

### Test Files

- **test_aes_gcm.rs**: Tests for AES-GCM encryption/decryption
  - Basic encryption/decryption without AAD
  - Encryption/decryption with Additional Authenticated Data (AAD)
  - Wrong key detection
  - Wrong AAD detection
  - Ciphertext manipulation detection
  - Ciphertext length verification
  - Empty message handling
  - Random IV verification

- **test_keystore.rs**: Tests for KeyStore functionality
  - Creating empty keystore
  - Adding and retrieving keys
  - Writing and reading keystore from file
  - Multiple keys management
  - Keystore persistence
  - Wrong password handling
  - Nonexistent key handling

- **test_stream.rs**: Tests for stream encryption/decryption
  - Small file encryption/decryption
  - Large file encryption/decryption (10KB+)
  - Empty file handling
  - Binary data handling
  - Wrong key detection
  - Base64 encoding verification
  - IV file creation and validation

- **integration_test.rs**: End-to-end integration tests
  - Complete workflow with keystore
  - Stream encryption with keystore integration
  - Multiple files with same keystore
  - Different AAD scenarios
  - Key overwrite behavior

### Running Tests

```bash
# Run all tests
cargo test -p exercise3

# Run specific test file
cargo test -p exercise3 --test test_aes_gcm
cargo test -p exercise3 --test test_keystore
cargo test -p exercise3 --test test_stream
cargo test -p exercise3 --test integration_test

# Run with output
cargo test -p exercise3 -- --nocapture

# Run specific test
cargo test -p exercise3 test_encrypt_decrypt_with_aad
```

## TODO

All functions are marked with `unimplemented!()` and need to be implemented as part of the exercise. Once implemented, run the tests to verify correctness.
