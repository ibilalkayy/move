use rusqlite::{Connection, Result};
use crate::cli::flags::cred::{BlockchainCred, GmailCred};

impl BlockchainCred {
    pub fn insert_blockchain(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into blockchain(private_key, alchemy_url) values($1, $2)",
            &[&self.private_key, &self.alchemy_url],
        )?;
        Ok(())
    }
}

impl GmailCred {
    pub fn insert_gmail(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "insert into gmail(username, gmail_address, app_password) values($1, $2, $3)",
            &[&self.username, &self.address, &self.app_password],
        )?;
        Ok(())
    }
}
