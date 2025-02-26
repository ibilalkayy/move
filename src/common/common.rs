use std::{fs, fs::File};
use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng}, Aes256Gcm,
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
            let encrypted_hex = hex::encode(&buffer);

            return encrypted_hex;
        }
        None => panic!("Err: no data is provided"),
    }
}