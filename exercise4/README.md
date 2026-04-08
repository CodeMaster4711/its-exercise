# Exercise 4: Digital Signatures with RSA

This exercise implements digital signatures using RSA with SHA-512 hashing in Rust.

## Modules

- **digital_signature.rs**: Sign and verify files using RSA digital signatures
- **crypto_util.rs**: Utility functions for reading/writing RSA keys and generating key pairs

## Features

- **Sign**: Sign file contents with RSA private key (SHA-512 + RSA)
- **Verify**: Verify digital signatures with RSA public key
- **Key Management**: Generate, read, and write RSA key pairs in PEM format

## Dependencies

- `rsa`: RSA encryption and signing
- `sha2`: SHA-512 hashing
- `rand`: Random number generation
- `anyhow`: Error handling
- `base64`: Base64 encoding/decoding
- `pem`: PEM file format handling

## Build

```bash
cargo build -p exercise4
```

## Run

```bash
cargo run -p exercise4
```

## TODO

All functions are marked with `unimplemented!()` and need to be implemented as part of the exercise.
