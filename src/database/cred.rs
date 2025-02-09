use rusqlite::{Connection, Result, ToSql};
use crate::cli::flags::cred::{BlockchainCred, GmailCred};
use tabled::{Table, Tabled};
use std::{fs, fs::File};
use std::process::exit;
use rusqlite::params;
use csv::Writer;

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
            println!("The blockchain credentials is already inserted");
            exit(0);
        }

        conn.execute(
            "insert into blockchain(private_key, alchemy_url) values(?1, ?2)",
            &[&self.private_key, &self.alchemy_url],
        )?;
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
            return Err(rusqlite::Error::InvalidQuery);
        }

        query.push_str(&field.join(", "));

        let affected_row = conn.execute(&query, rusqlite::params_from_iter(value))?;
        if affected_row == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    pub fn get_blockchain(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("select private_key, alchemy_url from blockchain")?;

        let rows = stmt.query_map(params![], |row| {
            Ok(BlockchainRow {
                private_key: row.get(0)?,
                alchemy_url: row.get(1)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?)
        }

        let home_dir = dirs::home_dir().expect("failed to get the home directory");
        let joined_dir = home_dir.join("move");

        if !joined_dir.exists() {
            fs::create_dir_all(&joined_dir).expect("Failed to create directory");
        }

        let merge_path = joined_dir.join("blockchain_data.csv");
        let file_path = File::create(merge_path).expect("failed to create a file");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Private Key", "Alchemy URL"]).unwrap();

        for blockchain in result {
            wtr.write_record(&[
                blockchain.private_key,
                blockchain.alchemy_url,
            ]).unwrap();
        }

        wtr.flush().unwrap();

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
            println!("The gmail credentials are already inserted");
            exit(0);
        }

        conn.execute(
            "insert into gmail(username, gmail_address, app_password) values(?1, ?2, ?3)",
            &[&self.username, &self.gmail_address, &self.app_password],
        )?;
        Ok(())
    }

    pub fn update_gmail(&self, conn: &Connection) -> Result<()> {
        let mut query = String::from("update gmail set ");
        let mut field = Vec::new();
        let mut value: Vec<&dyn ToSql> = Vec::new();

        if let Some(username) = &self.username {
            field.push("username = ?");
            value.push(username);
        }

        if let Some(gmail_address) = &self.gmail_address {
            field.push("gmail_address = ?");
            value.push(gmail_address);
        }

        if let Some(app_password) = &self.app_password {
            field.push("app_password = ?");
            value.push(app_password);
        }

        if field.is_empty() {
            return Err(rusqlite::Error::InvalidQuery);
        }

        query.push_str(&field.join(", "));

        let affected_row = conn.execute(&query, rusqlite::params_from_iter(value))?;
        if affected_row == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }

    pub fn get_gmail(&self, conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("select username, gmail_address, app_password from gmail")?;

        let rows = stmt.query_map(params![], |row| {
            Ok(GmailRow {
                username: row.get(0)?,
                gmail_address: row.get(1)?,
                app_password: row.get(2)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?)
        }

        let home_dir = dirs::home_dir().expect("failed to get the home directory");
        let joined_dir = home_dir.join("move");

        if !joined_dir.exists() {
            fs::create_dir_all(&joined_dir).expect("Failed to create directory");
        }

        let merge_path = joined_dir.join("gmail_data.csv");
        let file_path = File::create(merge_path).expect("failed to create a file");

        let mut wtr = Writer::from_writer(file_path);

        wtr.write_record(&["Username", "Gmail Address", "App Password"]).unwrap();

        for gmail in result {
            wtr.write_record(&[
                gmail.username,
                gmail.gmail_address,
                gmail.app_password,
            ]).unwrap();
        }

        wtr.flush().unwrap();

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

pub fn delete_blockchain(conn: &Connection) -> Result<()> {
    let affected_rows = conn.execute("DELETE FROM blockchain", [])?;
    
    if affected_rows == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
    }
    
    Ok(())
}

pub fn delete_gmail(conn: &Connection) -> Result<()> {
    let affected_rows = conn.execute("DELETE FROM gmail", [])?;
    
    if affected_rows == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
    }
    
    Ok(())
}