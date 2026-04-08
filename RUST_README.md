# ITS Exercises - Rust Implementation

Dieses Repository enthält die Rust-Implementierungen der ITS-Übungen (Information Technology Security).

## Workspace-Struktur

Das Projekt ist als Cargo Workspace organisiert mit drei Exercises:

### Exercise 2: AES Encryption (bereits vorhanden)
- Grundlegende AES-Verschlüsselung
- Datei-Utilities
- En- und Dekodierung

### Exercise 3: AES-GCM Encryption und KeyStore
**Module:**
- `encrypt_aes_gcm.rs` - AES-GCM Verschlüsselung mit AAD (Additional Authenticated Data)
- `decrypt_aes_gcm.rs` - AES-GCM Entschlüsselung
- `keystore_utils.rs` - Verwaltung von symmetrischen Schlüsseln (ähnlich PKCS12)
- `stream_encryption.rs` - Stream-basierte Verschlüsselung mit Base64
- `stream_decryption.rs` - Stream-basierte Entschlüsselung

**Dependencies:**
- `aes-gcm` - AES-GCM Verschlüsselung
- `base64` - Base64 Kodierung
- `rand` - Zufallszahlengenerierung
- `anyhow` - Fehlerbehandlung

### Exercise 4: Digitale Signaturen mit RSA
**Module:**
- `digital_signature.rs` - Signieren und Verifizieren von Dateien (SHA-512 + RSA)
- `crypto_util.rs` - RSA-Schlüsselverwaltung (PEM-Format)

**Dependencies:**
- `rsa` - RSA Verschlüsselung und Signierung
- `sha2` - SHA-512 Hashing
- `rand` - Zufallszahlengenerierung
- `pem` - PEM-Format Handling
- `anyhow` - Fehlerbehandlung

## Build und Run

```bash
# Gesamtes Workspace bauen
cargo build --workspace

# Einzelnes Exercise bauen
cargo build -p exercise2
cargo build -p exercise3
cargo build -p exercise4

# Einzelnes Exercise ausführen
cargo run -p exercise2
cargo run -p exercise3
cargo run -p exercise4

# Tests ausführen
cargo test --workspace
```

## Implementierung

Alle Funktionen in Exercise 3 und 4 sind mit `unimplemented!()` markiert und müssen als Teil der Übung implementiert werden. Die Struktur und Signaturen orientieren sich an den ursprünglichen Java-Implementierungen, sind aber in idiomatisches Rust übersetzt.

## Struktur-Konventionen

Die Rust-Implementierungen folgen den Konventionen von Exercise 2:
- Modulare Aufteilung in logische Einheiten
- `lib.rs` als öffentliche API
- Verwendung von `anyhow::Result` für Fehlerbehandlung
- Generische Pfad-Parameter `P: AsRef<Path>` für Dateioperationen
- Dokumentationskommentare für alle öffentlichen Funktionen
