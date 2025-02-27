use std::{fs, fs::File};
use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng}, Aes256Gcm, Nonce,
};
use hex;

pub fn create_file(path: &str) -> File {
    let home_dir = dirs::home_dir().expect("Err: failed to get the home directory");
    let joined_dir = home_dir.join("move");

    if !joined_dir.exists() {
        fs::create_dir_all(&joined_dir).expect("Err: failed to create directory");
    }

    let merge_path = joined_dir.join(path);
    let file_path = File::create(merge_path).expect("Err: failed to create a file");
    return file_path;
}

pub fn encrypt_data(data: Option<String>) -> String {
    match data {
        Some(text) => {
            // Generate encryption key and cipher
            let key = Aes256Gcm::generate_key(&mut OsRng);
            let cipher = Aes256Gcm::new(&key);
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

            // Convert plaintext to bytes
            let mut buffer = text.as_bytes().to_vec();

            // Encrypt the message in-place
            cipher.encrypt_in_place(&nonce, b"", &mut buffer)
                .expect("Err: encryption failed");

            // Store encrypted data as hex
            let encrypted_text = hex::encode(&buffer);
            let key_hex = hex::encode(key);
            let nonce_hex = hex::encode(nonce);

            println!("");
            println!("🗝️  Key: {}", key_hex);
            println!("🔑 Nonce: {}", nonce_hex);

            return encrypted_text;
        }
        None => panic!("Err: no data is provided"),
    }
}

pub fn decrypt_data(encrypted_hex: &str, key_hex: &str, nonce_hex: &str) {
    let key_bytes = hex::decode(key_hex).expect("Err: invalid hex key");
    let nonce_bytes = hex::decode(nonce_hex).expect("Err: invalid hex nonce");
    let mut encrypted_bytes = hex::decode(encrypted_hex).expect("Err: invalid hex ciphertext");

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let cipher = Aes256Gcm::new(key);

    cipher.decrypt_in_place(nonce, b"", &mut encrypted_bytes)
        .expect("Err: decryption failed");

    let decrypted_text = String::from_utf8_lossy(&encrypted_bytes);
    println!("🔓 Decrypted text: {}", decrypted_text);
    // decrypted_text.to_string()
}