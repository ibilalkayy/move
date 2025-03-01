use rusqlite::{Connection, Result};
use crate::common::common::decrypt_data;

fn credentials(conn: &Connection) -> Result<(String, String, String, String, u64), rusqlite::Error> {
    let mut stmt = conn.prepare("select private_key, private_key_nonce, alchemy_url, alchemy_url_nonce, chain_id from blockchain")?;
    
    let result = stmt.query_row([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
    })?;
    
    Ok(result)
}

pub fn give_data(conn: &Connection, keys: [String; 2]) -> Result<(String, String, u64), rusqlite::Error> {
    let (private_key, private_key_nonce, alchemy_url, alchemy_url_nonce, chain_id) = credentials(conn)?;

    let private_key = decrypt_data(private_key, keys[0].clone(), private_key_nonce);
    let alchemy_url = decrypt_data(alchemy_url, keys[1].clone(), alchemy_url_nonce);

    Ok((private_key, alchemy_url, chain_id))
}

pub fn cred_exists(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("select exists(select 1 from blockchain)")?;
    let exists: bool = stmt.query_row([], |row| row.get(0))?;
    Ok(exists)
}
