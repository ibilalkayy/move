use std::{fs, fs::File};

pub fn create_file(path: &str) -> File {
    let home_dir = dirs::home_dir().expect("Failed to get the home directory");
    let joined_dir = home_dir.join("move");

    if !joined_dir.exists() {
        fs::create_dir_all(&joined_dir).expect("Failed to create directory");
    }

    let merge_path = joined_dir.join(path);
    let file_path = File::create(merge_path).expect("Failed to create a file");
    return file_path;
}
