use std::{fs::OpenOptions, error::Error};
use csv::Writer;

pub fn insert_data(headers: &[&str], details: Vec<Vec<String>>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)?;

    let mut wtr = Writer::from_writer(file);

    let metadata = std::fs::metadata(file_path)?;
    if metadata.len() == 0 {
        wtr.write_record(headers)?;
    }

    for detail in details {
        wtr.write_record(&detail)?;
    }

    wtr.flush()?;
    
    Ok(())
}
