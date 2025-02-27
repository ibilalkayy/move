use rusqlite::{Connection, Result};
use crate::common::common::decrypt_data;

fn credentials(conn: &Connection) -> Result<(String, String, u64), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT private_key, alchemy_url, chain_id FROM blockchain")?;
    
    let result = stmt.query_row([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;
    
    Ok(result)
}

pub fn give_data(conn: &Connection, key_nonce: [&str; 4]) -> Result<(String, String, u64), rusqlite::Error> {
    let (private_key, alchemy_url, chain_id) = credentials(conn)?;

    let private_key = decrypt_data(private_key.as_str(), key_nonce[0], key_nonce[1]);
    let alchemy_url = decrypt_data(alchemy_url.as_str(), key_nonce[2], key_nonce[3]);

    Ok((private_key, alchemy_url, chain_id))
}