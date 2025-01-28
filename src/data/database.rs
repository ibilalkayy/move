use crate::cli::flags::{
    AddTotalAmount, AddTotalCategories, UpdateTotal, RemoveTotal, BudgetData, CreateBudget, 
    GetBudget, UpdateBudget, AlertData, AlertValues, SpendData,
};
use csv::Writer;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::error::Error;
use std::{env, fs, path::Path};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TotalAmountRow {
    #[tabled(rename="Total Amount")]
    total_amount: String,

    #[tabled(rename="Spent Amount")]
    spent_amount: String,

    #[tabled(rename="Remaining Amount")]
    remaining_amount: String,
}

#[derive(Tabled)]
struct TotalCategoryRow {
    #[tabled(rename="Categories")]
    category: String,

    #[tabled(rename="Labels")]
    label: String,

    #[tabled(rename="Statuses")]
    status: String,
}

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

pub fn view_total_amount() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    let mut rows = Vec::new();

    for row in client.query(
        "select total_amount, spent_amount, remaining_amount from totalamount",
        &[],
    )? {
        let total_amount: String = row.get(0);
        let spent_amount: String = row.get(1);
        let remaining_amount: String = row.get(2);

        rows.push(TotalAmountRow{
            total_amount,
            spent_amount,
            remaining_amount,
        });
        break;
    }

    let table = Table::new(rows);
    println!("{}", table);

    Ok(())
}

pub fn view_total_categories() -> Result<(), Box<dyn Error>> {
    let mut client = connection()?;
    let mut rows = Vec::new();

    for row in client.query(
        "select category, label, statuss from totalamount",
        &[],
    )? {
        let category: String = row.get(0);
        let label: String = row.get(1);
        let status: String = row.get(2);

        rows.push(TotalCategoryRow{
            category,
            label,
            status
        });
    }

    let table = Table::new(rows);
    println!("{}", table);

    Ok(())
}

impl AddTotalCategories {
    pub fn insert_total_categories(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into totalcategories(category, label, statuss) values($1, $2, $3)",
            &[&self.category, &self.label, &"inactive"],
        )?;
        Ok(())
    }
}

impl AddTotalAmount {
    pub fn insert_total_amount(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into totalamount(total_amount, spent_amount, remaining_amount) values($1, $2, $3)",
            &[&self.amount, &"0", &"0"],
        )?;
        Ok(())
    }
}

impl UpdateTotal {
    pub fn update_total(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "update totalamount set category=$1, total_amount=$2, label=$3 where category=$4",
            &[&self.new_category, &self.amount, &self.label, &self.old_category],
        )?;
        Ok(())
    }
}

impl RemoveTotal {
    pub fn remove_total(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute("delete from totalamount where category=$1", &[&self.category])?;
        Ok(())
    }
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

impl AlertData {
    pub fn create_alert(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute(
            "insert into alert(
                category,
                frequency,
                method,
                dayz,
                hourz,
                minutez,
                secondz,
                weekdays
            ) values($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &self.category, 
                &self.frequency,
                &self.method,
                &self.day,
                &self.hour,
                &self.minute,
                &self.second,
                &self.weekday,
            ],
        )?;
        Ok(())
    }

    pub fn update_data(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        if let Some(old_category) = &self.old_category {
            let _ = client.execute(
                "update alert set 
                category=$1,
                frequency=$2,
                method=$3,
                dayz=$4,
                hourz=$5,
                minutez=$6,
                secondz=$7,
                weekdays=$8
                where category=$9",
                &[
                    &self.category, 
                    &self.frequency,
                    &self.method,
                    &self.day,
                    &self.hour,
                    &self.minute,
                    &self.second,
                    &self.weekday,
                    &old_category,
                ],
            )?;
            Ok(())
        } else {
            Err("Old category is required to update the alert".into())
        }
    }
}

impl AlertValues {
    pub fn get_alert(&self) {
        println!("Get your alert notifications through email");
    }

    pub fn see_alert(&self) {
        println!("See your alert notifications in the terminal");
    }

    pub fn remove_alert(&self) -> Result<(), Box<dyn Error>> {
        let mut client = connection()?;
        let _ = client.execute("delete from alert where category=$1", &[&self.category])?;
        Ok(())
    }
}

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