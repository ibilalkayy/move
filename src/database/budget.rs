use crate::cli::flags::budget::{BudgetData, CreateBudget, GetBudget, UpdateBudget};
use crate::database::db::connection;
use std::error::Error;
use csv::Writer;

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

pub fn list_data() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    for row in client.query("select category, amount from budget", &[])? {
        let category: String = row.get(0);
        let amount: String = row.get(1);

        println!("Category: {}\nAmount: {}", category, amount);
    }
    Ok(())
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
