use std::fs;
use std::path::Path;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::rand_core::RngCore;

pub fn encrypt_password(plain_text: &str, key: &[u8; 32]) -> (String, String) {
    let cipher = Aes256Gcm::new_from_slice(key).expect("invalid key");

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plain_text.as_bytes())
        .expect("encryption failed");

    (hex::encode(nonce_bytes), hex::encode(ciphertext))
}

pub fn decrypt_password(nonce_hex: &str, ciphertext_hex: &str, key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new_from_slice(key)
        .expect("Invalid key length");

    let nonce_vec = hex::decode(nonce_hex)
        .expect("Invalid nonce hex");
    let ciphertext = hex::decode(ciphertext_hex)
        .expect("Invalid ciphertext hex");

    let nonce = Nonce::from_slice(&nonce_vec);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("Decryption failed");

    String::from_utf8(plaintext)
        .expect("Plaintext was not valid UTF-8")
}

pub fn load_or_create_key() -> [u8; 32] {
    let key_path = "secret.key";
    if Path::new(&key_path).exists() {
        let key_hex = fs::read_to_string(key_path).expect("Failed to read key file");
        let key_bytes = hex::decode(&key_hex.trim()).expect("Failed to parse hex key file");
        let key_array: [u8; 32] = key_bytes.try_into().expect("Failed to convert key file");

        key_array
    } else {
        let mut key = [0; 32];
        OsRng.fill_bytes(&mut key);
        fs::write(key_path, hex::encode(key)).expect("Failed to write key file");
        key
    }
}
