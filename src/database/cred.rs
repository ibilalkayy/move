use crate::cli::flags::cred::BlockchainCred;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};
use crate::common::common::encrypt_data;
// use crate::middleware::middleware::http_provider;

#[derive(Tabled)]
struct BlockchainRow {
    #[tabled(rename = "Private Key")]
    private_key: String,

    #[tabled(rename = "Alchemy URL")]
    alchemy_url: String,
}

impl BlockchainCred {
    pub fn insert_blockchain(&self, conn: &Connection) -> Result<()> {
        let row_exists: bool = conn.query_row(
            "select exists (select 1 from blockchain)",
            params![],
            |row| row.get(0),
        )?;

        if row_exists {
            panic!("Err: multiple insertions of blockchain credentials is not allowed");
        }

        let private_key_data = encrypt_data(self.private_key.clone()); 
        let alchemy_url_data = encrypt_data(self.alchemy_url.clone()); 

        match self.private_key {
            Some(_) => {
                conn.execute(
                    "insert into blockchain(private_key, alchemy_url, chain_id) values(?1, ?2, ?3)",
                    (&private_key_data, &alchemy_url_data, &self.chain_id),
                )
                .expect("Err: failed to execute");
            }
            None => panic!("Err: cred is not added yet. See 'move cred -h'"),
        }
        Ok(())
    }

    pub fn update_blockchain(&self, conn: &Connection) -> Result<()> {
        let mut query = String::from("update blockchain set ");
        let mut field = Vec::new();
        let mut value: Vec<&dyn ToSql> = Vec::new();

        if let Some(private_key) = &self.private_key {
            field.push("private_key = ?");
            value.push(private_key);
        }

        if let Some(alchemy_url) = &self.alchemy_url {
            field.push("alchemy_url = ?");
            value.push(alchemy_url);
        }

        if field.is_empty() {
            panic!("Err: cred is not added yet. See 'move cred -h'");
        }

        query.push_str(&field.join(", "));

        let affected_row = conn.execute(&query, rusqlite::params_from_iter(value));
        if affected_row == Ok(0) {
            return Err(rusqlite::Error::QueryReturnedNoRows);
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
