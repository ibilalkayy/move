use crate::cli::flags::budget::{BudgetCategory, BudgetData, UpdateBudget};
use crate::common::common::create_file;
use crate::usecases::{
    budget::{budget_category_exists, budget_total_equal, convert_to_u64},
    total_amount::total_amount_exists,
    total_categories::{total_category_exists, total_categories_exist},
};
use csv::Writer;
use rusqlite::{params, Connection, Result, ToSql};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct BudgetRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

impl BudgetData {
    pub fn insert_budget(&self, conn: &Connection) -> Result<()> {
        let find_category = total_category_exists(conn, &self.category);
        let find_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, &self.category);
        let is_budget_total_equal = budget_total_equal(conn, "");

        match find_category {
            Ok(true) => {
                match find_amount {
                    Ok(true) => {
                        match find_budget_category {
                            Ok(true) => panic!("Category {} is already present in the record", &self.category),
                            Ok(false) => {
                                match is_budget_total_equal {
                                    Ok((total_amount, budget_total_sum, _, _)) => {
                                        let given_amount = self.amount.parse::<u64>().unwrap();
                                        let budget_amount = budget_total_sum + given_amount;
            
                                        if budget_amount <= total_amount {
                                            conn.execute(
                                                "insert into budget(category, amount) values(?1, ?2)",
                                                &[&self.category, &self.amount],
                                            )?;
                                        } else {
                                            panic!(
                                                "Budget amount exceeded the total amount: {} > {}",
                                                budget_amount, total_amount
                                            );
                                        }
                                    }
                                    Err(error) => panic!("Err: {}", error),
                                }
                            },
                            Err(error) => panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Total amount is not added in the record yet"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!("Category {} is not present in the total categories list. First add one.", self.category),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }

    pub fn get_budget(&self, conn: &Connection) -> Result<()> {
        let find_category = total_category_exists(conn, &self.category);
        match find_category {
            Ok(true) => {
                let mut stmt = conn.prepare("select category, amount from budget")?;

                let rows = stmt.query_map(params![], |row| {
                    Ok(BudgetRow {
                        category: row.get(0)?,
                        amount: row.get(1)?,
                    })
                })?;

                let mut results = Vec::new();
                for row in rows {
                    results.push(row?);
                }
                let file_path = create_file("budget.csv");

                let mut wtr = Writer::from_writer(file_path);
                wtr.write_record(&["Category", "Amount"])
                    .expect("failed to write the data in a CSV file");

                for budget in results {
                    wtr.write_record(&[budget.category, budget.amount])
                        .expect("failed to write the data in a CSV file");
                }
                wtr.flush().expect("failed to flush the content");
            }
            Ok(false) => panic!("No category is present to get"),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}

impl BudgetCategory {
    pub fn view_budget(&self, conn: &Connection, category: &str) -> Result<()> {
        let find_category = total_category_exists(conn, &self.category);
        match find_category {
            Ok(true) => {
                let mut stmt = conn.prepare("select category, amount from budget where category = ?")?;

                let rows = stmt.query_map(params![category], |row| {
                    Ok(BudgetRow {
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
            Ok(false) => panic!("No category is present to be viewed"),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }

    pub fn delete_budget(&self, conn: &Connection) -> Result<()> {
        let find_category = total_category_exists(conn, &self.category);
        match find_category {
            Ok(true) => {
                let affected_rows =
                conn.execute("delete from budget where category = ?", &[&self.category])?;
    
                if affected_rows == 0 {
                    return Err(rusqlite::Error::QueryReturnedNoRows); // No rows were deleted
                }
            }
            Ok(false) => panic!("No category is present to be deleted"),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}

pub fn show_budget(conn: &Connection) -> Result<()> {
    let find_category = total_categories_exist(conn);
    match find_category {
        Ok(true) => {
            let mut stmt = conn.prepare("select category, amount from budget")?;

            let rows = stmt.query_map(params![], |row| {
                Ok(BudgetRow {
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
        Ok(false) => panic!("No category is present to be shown"),
        Err(error) => panic!("Err: {}", error),
    }
    Ok(())
}

impl UpdateBudget {
    pub fn update_budget(&self, conn: &Connection) -> Result<()> {
        let new_category: &str = self.new_category.as_deref().unwrap_or("");

        let mut query = String::from("update budget set ");
        let mut fields = Vec::new();
        let mut value: Vec<&dyn ToSql> = Vec::new();

        if let Some(new_category) = &self.new_category {
            fields.push("category = ?");
            value.push(new_category);
        }

        if let Some(amount) = &self.amount {
            fields.push("amount = ?");
            value.push(amount);
        }

        if fields.is_empty() {
            return Err(rusqlite::Error::InvalidQuery);
        }

        query.push_str(&fields.join(", "));
        query.push_str("where category = ?");

        value.push(&self.old_category);

        let find_new_category = total_category_exists(conn, new_category);
        let find_old_category = total_category_exists(conn, &self.old_category);
        let find_total_amount = total_amount_exists(conn);
        let find_budget_old_category = budget_category_exists(conn, &self.old_category);
        let find_budget_new_category = budget_category_exists(conn, new_category);
        let is_budget_total_equal = budget_total_equal(conn, &self.old_category);

        match find_new_category {
            Ok(true) => {
                match find_old_category {
                    Ok(true) => {
                        match find_total_amount {
                            Ok(true) => {
                                match find_budget_old_category {
                                    Ok(true) => {
                                        match find_budget_new_category {
                                            Ok(true) => panic!("Category {} is already present in the record", new_category),
                                            Ok(false) => {
                                                match is_budget_total_equal {
                                                    Ok((total_amount, _, budget_except_sum, _)) => {
                                                        let given_amount = convert_to_u64(self.amount.clone());
                                                        let budget_amount = budget_except_sum + given_amount;
                        
                                                        if budget_amount <= total_amount {
                                                            let affected_rows = conn.execute(&query, rusqlite::params_from_iter(value))?;
                                                            if affected_rows == 0 {
                                                                return Err(rusqlite::Error::QueryReturnedNoRows);
                                                            }
                                                        } else {
                                                            panic!(
                                                                "Budget amount exceeded the total amount: {} > {}",
                                                                budget_amount, total_amount,
                                                            );
                                                        }
                                                    }
                                                    Err(error) => panic!("Err: {}", error),
                                                }                                                
                                            },
                                            Err(error) => panic!("Err: {}", error),
                                        }
                                    }
                                    Ok(false) => panic!(
                                        "Category {} is not present in the budget record. First add it to a budget",
                                        &self.old_category
                                    ),
                                    Err(error) =>  panic!("Err: {}", error),
                                }
                            }
                            Ok(false) => panic!("Total amount is not present in the record"),
                            Err(error) =>  panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Category {} is not present in the old categories list", self.old_category),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!("Category {} is not present in the new categories list. First add one", new_category),
            Err(error) => panic!("Err: {}", error),
        }
        Ok(())
    }
}
