use dotenv::dotenv;
use postgres::{Client, NoTls, Error};
use std::{fs, env, path::Path};
use crate::cli::flags::{CreateBudget, BudgetData};

fn connection() -> Result<Client, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set in .env");
    let client = Client::connect(&database_url, NoTls)?;
    Ok(client)
}

pub fn create_table() -> Result<(), Error> {
    let mut client = connection()?;
    let sql_file_path = Path::new("sql/create_table.sql");
    let sql_query = fs::read_to_string(sql_file_path).expect("Failed to read the SQL file");
    client.batch_execute(&sql_query)?;
    Ok(())
}

impl CreateBudget {
    pub fn insert_data(&self) -> Result<(), Error> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into budget(category, amount) values($1, $2::BIGINT)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }
}

impl BudgetData {
    pub fn view_data(&self) -> Result<(), Error> {
        let mut client = connection()?;
        for row in client.query("select category, amount::BIGINT from budget where category=$1", &[&self.category])? {
            let category: String = row.get(0);
            let amount: i64 = row.get(1);

            println!("Category: {}\nAmount: {}", category, amount);
        }
        Ok(())
    }

    pub fn delete_data(&self) -> Result<String, Error> {
        let mut client = connection()?;
        let _ = client.execute(
            "delete from budget where category=$1",
            &[&self.category],
        )?;
        Ok(self.category.clone())
    }
}
