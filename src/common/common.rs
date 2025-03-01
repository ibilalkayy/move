use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use hex;
use std::{fs, fs::File};

pub fn create_file(path: &str) -> File {
    let home_dir = dirs::home_dir().expect("❌ Failed to get a home directory");
    let joined_dir = home_dir.join("move");

    if !joined_dir.exists() {
        fs::create_dir_all(&joined_dir).expect("❌ Failed to create a directory");
    }

    let merge_path = joined_dir.join(path);
    let file_path = File::create(merge_path).expect("❌ Failed to create a file");
    return file_path;
}

pub fn encrypt_data(data: String) -> (String, String) {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut buffer = data.as_bytes().to_vec();
    cipher
        .encrypt_in_place(&nonce, b"", &mut buffer)
        .expect("❌ Encryption failed");

    let encrypted_text = hex::encode(&buffer);
    let key_hex = hex::encode(key);
    let nonce_hex = hex::encode(nonce);

    println!("");
    println!("🗝️  Key: {}", key_hex);

    return (encrypted_text, nonce_hex);
}

pub fn decrypt_data(encrypted_hex: String, key_hex: String, nonce_hex: String) -> String {
    let key_bytes = hex::decode(key_hex).expect("❌ Invalid hex key");
    let nonce_bytes = hex::decode(nonce_hex).expect("❌ Invalid hex nonce");
    let mut encrypted_bytes = hex::decode(encrypted_hex).expect("❌ Invalid hex ciphertext");

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let cipher = Aes256Gcm::new(key);

    cipher
        .decrypt_in_place(nonce, b"", &mut encrypted_bytes)
        .expect("❌ Decryption failed");

    let decrypted_text = String::from_utf8_lossy(&encrypted_bytes);
    decrypted_text.to_string()
}
