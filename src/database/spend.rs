use crate::cli::flags::spend::SpendData;
use crate::database::db::connection;
use std::error::Error;

impl SpendData {
    pub fn insert_spending(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into spend(category, amount) values($1, $2)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }
}
