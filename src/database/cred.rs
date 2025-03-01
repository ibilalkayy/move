use crate::cli::flags::cred::BlockchainCred;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};
use crate::common::common::encrypt_data;
use crate::usecases::cred::cred_exists;

#[derive(Tabled)]
struct BlockchainRow {
    #[tabled(rename = "Private Key")]
    private_key: String,

    #[tabled(rename = "Alchemy URL")]
    alchemy_url: String,
}

impl BlockchainCred {
    pub fn insert_blockchain(&self, conn: &Connection) -> Result<()> {
        let row_exists = cred_exists(conn)?;

        if row_exists {
            panic!("Err: multiple insertions of blockchain credentials is not allowed");
        }

        let (private_key_data, private_key_nonce) = encrypt_data(self.private_key.clone()); 

        match self.alchemy_url.clone() {
            Some(alchemy_url) => {
                let (alchemy_url_data, alchemy_url_nonce) = encrypt_data(alchemy_url.clone());   
                conn.execute(
                    "insert into blockchain(private_key, private_key_nonce, alchemy_url, alchemy_url_nonce, chain_id) values(?1, ?2, ?3, ?4, ?5)",
                    (&private_key_data, &private_key_nonce, &alchemy_url_data, &alchemy_url_nonce, &self.chain_id),
                ).expect("Err: failed to execute");
            }
            None => {
                conn.execute(
                    "insert into blockchain(private_key, private_key_nonce, chain_id) values(?1, ?2, ?3)",
                    (&private_key_data, &private_key_nonce, &self.chain_id),
                ).expect("Err: failed to execute");
            }
        }
        
        Ok(())
    }

    pub fn update_blockchain(&self, conn: &Connection) -> Result<()> {
        let row_exists = cred_exists(conn)?;

        if !row_exists {
            panic!("Err: cred is not added yet. See move cred -h'");
        }

        let (private_key_data, private_key_nonce) = encrypt_data(self.private_key.clone()); 
        match self.alchemy_url.clone() {
            Some(alchemy_url) => {
                let (alchemy_url_data, alchemy_url_nonce) = encrypt_data(alchemy_url.clone());   
                conn.execute(
                    "update blockchain set private_key = ?, private_key_nonce = ?, alchemy_url = ?, alchemy_url_nonce = ?, chain_id = ?", 
                    (&private_key_data, &private_key_nonce, &alchemy_url_data, &alchemy_url_nonce, &self.chain_id),
                ).expect("Err: failed to update the status");
            }
            None => {
                conn.execute(
                    "update blockchain set private_key = ?, private_key_nonce = ?, chain_id = ?", 
                    (&private_key_data, &private_key_nonce, &self.chain_id),
                ).expect("Err: failed to update the status");
            }
        }
    
        Ok(())
    }
    
}

pub fn view_blockchain(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select private_key, alchemy_url from blockchain")?;

    let rows = stmt.query_map(params![], |row| {
        Ok(BlockchainRow {
            private_key: row.get(0)?,
            alchemy_url: row.get(1)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    let table = Table::new(results);
    println!("{}", table);

    Ok(())
}

pub fn delete_blockchain(conn: &Connection) -> Result<()> {
    let affected_rows = conn.execute("delete from blockchain", [])?;

    if affected_rows == 0 {
        panic!("Err: cred is not added yet. See 'move cred -h'");
    }

    Ok(())
}
