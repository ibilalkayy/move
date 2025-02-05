use std::{error::Error, fs, fs::OpenOptions};
use csv::Writer;

pub fn insert_data(headers: &[&str], details: Vec<Vec<String>>, file_name: &str, message: &str) -> Result<(), Box<dyn Error>> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let move_dir = home_dir.join("move");
    fs::create_dir_all(&move_dir)?; // Ensure the "move" directory exists

    // Save the file inside "move" directory
    let file_path = move_dir.join(file_name);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_path)?;

    let mut wtr = Writer::from_writer(file);

    let metadata = std::fs::metadata(&file_path)?;
    if metadata.len() == 0 {
        wtr.write_record(headers)?;
    }

    for detail in details {
        wtr.write_record(&detail)?;
    }

    wtr.flush()?;
    
    println!("{} data is successfully stored at: {:?}", message, file_path);

    Ok(())
}
