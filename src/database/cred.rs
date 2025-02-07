use rusqlite::{Connection, Result};
use crate::cli::flags::cred::{BlockchainCred, GmailCred};
use rusqlite::params;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct BlockchainRow {
    #[tabled(rename = "Private Key")]
    private_key: String,

    #[tabled(rename = "Alchemy URL")]
    alchemy_url: String,
}

#[derive(Tabled)]
struct GmailRow {
    #[tabled(rename = "Username")]
    username: String,

    #[tabled(rename = "Gmail Address")]
    gmail_address: String,

    #[tabled(rename = "App Password")]
    app_password: String,

}

impl BlockchainCred {
    pub fn insert_blockchain(&self, conn: &Connection) -> Result<()> {
        let row_exists: bool = conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM blockchain)",
            params![],
            |row| row.get(0),
        )?;

        if row_exists {
            println!("The blockchain data is already inserted");
            return Ok(());
        }

        conn.execute(
            "insert into blockchain(private_key, alchemy_url) values(?1, ?2)",
            &[&self.private_key, &self.alchemy_url],
        )?;
        Ok(())
    }
}

impl GmailCred {
    pub fn insert_gmail(&self, conn: &Connection) -> Result<()> {
        let row_exists: bool = conn.query_row(
            "SELECT EXISTS (SELECT 1 FROM gmail)",
            params![],
            |row| row.get(0),
        )?;

        if row_exists {
            println!("The gmail data is already inserted");
            return Ok(());
        }

        conn.execute(
            "insert into gmail(username, gmail_address, app_password) values(?1, ?2, ?3)",
            &[&self.username, &self.gmail_address, &self.app_password],
        )?;
        Ok(())
    }
}

pub fn view_blockchain(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT private_key, alchemy_url FROM blockchain",
    )?;

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

pub fn view_gmail(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT username, gmail_address, app_password FROM gmail",
    )?;

    let rows = stmt.query_map(params![], |row| {
        Ok(GmailRow {
            username: row.get(0)?,
            gmail_address: row.get(1)?,
            app_password: row.get(2)?,
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