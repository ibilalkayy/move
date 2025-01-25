use crate::cli::flags::{BudgetData, CreateBudget, GetBudget, UpdateBudget};
use csv::Writer;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::error::Error;
use std::{env, fs, path::Path};

fn connection() -> Result<Client, Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set in .env");
    let client = Client::connect(&database_url, NoTls)?;
    Ok(client)
}

pub fn create_table() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    let sql_file_path = Path::new("sql/create_table.sql");
    let sql_query = fs::read_to_string(sql_file_path).expect("Failed to read the SQL file");
    client.batch_execute(&sql_query)?;
    Ok(())
}

pub fn list_data() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    for row in client.query("select category, amount from budget", &[])? {
        let category: String = row.get(0);
        let amount: String = row.get(1);

        println!("Category: {}\nAmount: {}", category, amount);
    }
    Ok(())
}

impl CreateBudget {
    pub fn insert_data(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into budget(category, amount) values($1, $2)",
            &[&self.category, &self.amount],
        )?;
        Ok(())
    }
}

impl BudgetData {
    pub fn view_data(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        for row in client.query(
            "select category, amount from budget where category=$1",
            &[&self.category],
        )? {
            let category: String = row.get(0);
            let amount: String = row.get(1);

            println!("Category: {}\nAmount: {}", category, amount);
        }
        Ok(())
    }

    pub fn delete_data(&self) -> Result<String, Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute("delete from budget where category=$1", &[&self.category])?;
        Ok(self.category.clone())
    }
}

impl GetBudget {
    pub fn get_data(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let mut category: Vec<String> = Vec::new();
        let mut amount: Vec<String> = Vec::new();

        for row in client.query("select category, amount from budget", &[])? {
            category.push(row.get(0));
            amount.push(row.get(1));
        }

        let file_path_name = format!("{}/{}", self.filepath, self.filename);
        let mut wtr = Writer::from_path(file_path_name)?;

        wtr.write_record(&["Category", "Amount"])?;
        for (cat, amt) in category.iter().zip(amount.iter()) {
            wtr.write_record(&[cat, amt])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

impl UpdateBudget {
    pub fn update_data(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "update budget set category=$1, amount=$2 where category=$3",
            &[&self.new_category, &self.amount, &self.old_category],
        )?;
        Ok(())
    }
}
