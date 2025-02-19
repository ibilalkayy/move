use crate::cli::flags::spend::{SpendCategory, SpendData};
use crate::common::common::create_file;
use csv::Writer;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};
use crate::usecases::{
    budget::{budget_category_exists, budget_amount, spend_sum, get_budget_amount},
    total_amount::total_amount_exists,
    total_categories::total_category_exists,
};

#[derive(Tabled)]
struct SpendingRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

impl SpendData {
    pub fn insert_spending(&self, conn: &Connection) -> Result<()> {
        let category = self.category.as_deref().unwrap_or("");
        let find_total_category = total_category_exists(conn, category);
        let find_total_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, category);
        let budget_amount_data= budget_amount(conn, category);
        let spending_amount: Option<u64> = self.amount.as_deref().and_then(|s| s.parse::<u64>().ok());
        let spend_sum = spend_sum(conn, category);
        let budgetamount = get_budget_amount(conn, category);

        match find_total_category {
            Ok(true) => {
                match find_total_amount {
                    Ok(true) => {
                        match find_budget_category {
                            Ok(true) => {
                                match budget_amount_data {
                                    Ok(budget_amount) => {
                                        match spending_amount {
                                            Some(spend_amount) => {
                                                if spend_amount <= budget_amount {
                                                    match spend_sum {
                                                        Ok(sum_of_spend) => {
                                                            let added_spend_amount = sum_of_spend+spend_amount;
                                                            match budgetamount {
                                                                Ok(amount_of_budget) => {
                                                                    if added_spend_amount <= amount_of_budget {
                                                                        conn.execute(
                                                                            "insert into spend(category, amount) values(?1, ?2)",
                                                                            &[&self.category, &self.amount],
                                                                        )?;
                                                                    } else {
                                                                        panic!("Spending amount exceeded the budget amount");
                                                                    }
                                                                }
                                                                Err(error) => panic!("Err: {}", error),
                                                            } 
                                                        }
                                                        Err(error) => panic!("Err: {}", error),
                                                    }
                                                } else {
                                                    panic!("Category is not present in the budget record");
                                                }
                                            },
                                            None => panic!("Spending amount is not given"),
                                        }
                                    },
                                    Err(error) => panic!("Err: {}", error),
                                } 
                            }
                            Ok(false) => panic!("Category {} is not present in the budget list", category),
                            Err(error) => panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Amount is not present in the total amount list"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!("Category {} is not present in the total categories list", category),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }

    pub fn get_spending(&self, conn: &Connection) -> Result<()> {
        let category = self.category.as_deref().unwrap_or("");
        let find_total_category = total_category_exists(conn, category);
        let find_total_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, category);

        match find_total_category {
            Ok(true) => {
                match find_total_amount {
                    Ok(true) => {
                        match find_budget_category {
                            Ok(true) => {
                                let mut stmt = conn.prepare("select category, amount from spend")?;

                                let rows = stmt.query_map(params![], |row| {
                                    Ok(SpendingRow {
                                        category: row.get(0)?,
                                        amount: row.get(1)?,
                                    })
                                })?;
                        
                                let mut result = Vec::new();
                                for row in rows {
                                    result.push(row?)
                                }
                        
                                let file_path = create_file("spend.csv");
                        
                                let mut wtr = Writer::from_writer(file_path);
                                wtr.write_record(&["Category", "Amount"])
                                    .expect("failed to write the data in a CSV file");
                        
                                for spending in result {
                                    wtr.write_record(&[spending.category, spending.amount])
                                        .expect("failed to write the data in a CSV file");
                                }
                        
                                wtr.flush().expect("failed to flush the content");
                            }
                            Ok(false) => panic!("Category {} is not present in the budget list", category),
                            Err(error) => panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Amount is not present in the total amount list"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!("Category {} is not present in the total categories list", category),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}

impl SpendCategory {
    pub fn view_spending(&self, conn: &Connection, category: &str) -> Result<()> {
        let find_total_category = total_category_exists(conn, category);
        let find_total_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, category);

        match find_total_category {
            Ok(true) => {
                match find_total_amount {
                    Ok(true) => {
                        match find_budget_category {
                            Ok(true) => {
                                let mut stmt = conn.prepare("select category, amount from spend where category=?")?;

                                let rows = stmt.query_map(params![&category], |row| {
                                    Ok(SpendingRow {
                                        category: row.get(0)?,
                                        amount: row.get(1)?,
                                    })
                                })?;
                        
                                let mut results = Vec::new();
                                for row in rows {
                                    results.push(row?);
                                }
                        
                                let table = Table::new(results);
                                println!("{}", table);
                            }
                            Ok(false) => panic!("Category {} is not present in the budget list", category),
                            Err(error) => panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Amount is not present in the total amount list"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!("Category {} is not present in the total categories list", category),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }

    pub fn delete_spending(&self, conn: &Connection) -> Result<()> {
        let find_total_category = total_category_exists(conn, &self.category.as_str());
        let find_total_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, &self.category.as_str());

        match find_total_category {
            Ok(true) => {
                match find_total_amount {
                    Ok(true) => {
                        match find_budget_category {
                            Ok(true) => {
                                let affected_rows =
                                conn.execute("delete from spend where category = ?", &[&self.category])?;
                    
                                if affected_rows == 0 {
                                    return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
                                }
                            }
                            Ok(false) => panic!("Category {} is not present in the budget list", &self.category.as_str()),
                            Err(error) => panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Amount is not present in the total amount list"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!("Category {} is not present in the total categories list", &self.category.as_str()),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}
