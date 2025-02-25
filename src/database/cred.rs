use crate::cli::flags::cred::BlockchainCred;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};

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
            panic!("Err: inserting the blockchain credentials multiple times is not allowed");
        }

        match self.private_key {
            Some(_) => {
                conn.execute(
                    "insert into blockchain(private_key, alchemy_url) values(?1, ?2)",
                    &[&self.private_key, &self.alchemy_url],
                ).expect("Err: failed to execute");
            },
            None => panic!("Err: enter the cred. For more help, write: 'move cred add -h'"),
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
            panic!("Err: enter the cred first. For more help, write: 'move cred add -h'");
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
        panic!("Err: enter the cred first. For more help, write: 'move cred add -h'");
    }

    Ok(())
}