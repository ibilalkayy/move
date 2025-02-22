use crate::cli::flags::spend::{SpendCategory, SpendData};
use crate::common::common::create_file;
use crate::usecases::{
    alert::alerts_exist,
    budget::{budget_amount, budget_category_exists, get_budget_amount, spend_sum},
    total_amount::total_amount_exists,
    total_categories::total_category_exists,
    status::status,
};
use csv::Writer;
use rusqlite::{params, Connection, Result};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct SpendingRow {
    #[tabled(rename = "Category")]
    category: String,

    #[tabled(rename = "Amount")]
    amount: String,
}

impl SpendData {
    pub fn insert_spending(&self, conn: &Connection) -> Result<()> {
        // limiting the expenditure options
        let category = self.category.as_deref().unwrap_or("");
        let find_total_category = total_category_exists(conn, category);
        let find_total_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, category);
        let budget_amount_data = budget_amount(conn, category);
        let spend_sum = spend_sum(conn, category);
        let budgetamount = get_budget_amount(conn, category);
        let status = status(conn);
        let find_alert = alerts_exist(conn);

        // calculating the total, remaining and spent amounts
        // let string_spent_amount = self.amount.clone().unwrap_or_else(|| "0".to_string());

        // update_total_spent(conn, string_spent_amount, category);

        match find_total_category {
            Ok(true) => match find_total_amount {
                Ok(true) => match find_budget_category {
                    Ok(true) => match budget_amount_data {
                        Ok(budget_amount) => match self.amount {
                            Some(spending_amount) => {
                                if spending_amount <= budget_amount {
                                    match spend_sum {
                                        Ok(sum_of_spend) => {
                                            let added_spend_amount = sum_of_spend + spending_amount;
                                            match budgetamount {
                                                Ok(amount_of_budget) => {
                                                    if added_spend_amount <= amount_of_budget {
                                                        match find_alert {
                                                            Ok(true) => {
                                                                match status {
                                                                    Ok(status) => {
                                                                        if status == "active" {
                                                                            conn.execute(
                                                                                "insert into spend(category, amount) values(?1, ?2)",
                                                                                (&self.category, spending_amount),
                                                                            )?;
                                                                            println!("Money is spent successfully on the {} category", category);
                                                                        } else {
                                                                            panic!("Err: The status is not active yet");
                                                                        }
                                                                    }
                                                                    Err(error) => panic!("Err: {}", error),
                                                                }
                                                            }
                                                            Ok(false) => panic!("Err: Alert data is not found. First set the alert"),
                                                            Err(error) => panic!("Err: {}", error),
                                                        }
                                                    } else {
                                                        panic!("Err: Spending amount exceeded the budget amount");
                                                    }
                                                }
                                                Err(error) => panic!("Err: {}", error),
                                            }
                                        }
                                        Err(error) => panic!("Err: {}", error),
                                    }
                                } else {
                                    panic!("Err: Spending amount exceeded the budget amount");
                                }
                            }
                            None => panic!("Err: Spending amount is not given"),
                        },
                        Err(error) => panic!("Err: {}", error),
                    },
                    Ok(false) => panic!(
                        "Err: {} category is not present in the budget list",
                        category
                    ),
                    Err(error) => panic!("Err: {}", error),
                },
                Ok(false) => panic!("Err: Amount is not present in the total amount list"),
                Err(error) => panic!("Err: {}", error),
            },
            Ok(false) => panic!(
                "Err: {} category is not present in the total categories list",
                category
            ),
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
            Ok(true) => match find_total_amount {
                Ok(true) => match find_budget_category {
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
                    Ok(false) => panic!(
                        "Err: {} category is not present in the budget list",
                        category
                    ),
                    Err(error) => panic!("Err: {}", error),
                },
                Ok(false) => panic!("Err: Amount is not present in the total amount list"),
                Err(error) => panic!("Err: {}", error),
            },
            Ok(false) => panic!(
                "Err: {} category is not present in the total categories list",
                category
            ),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}

impl SpendCategory {
    pub fn view_spending(&self, conn: &Connection) -> Result<()> {
        let find_total_category = total_category_exists(conn, &self.category);
        let find_total_amount = total_amount_exists(conn);
        let find_budget_category = budget_category_exists(conn, &self.category);

        match find_total_category {
            Ok(true) => match find_total_amount {
                Ok(true) => match find_budget_category {
                    Ok(true) => {
                        let mut stmt =
                            conn.prepare("select category, amount from spend where category=?")?;

                        let rows = stmt.query_map(params![&self.category], |row| {
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
                    Ok(false) => panic!(
                        "Err: {} category is not present in the budget list",
                        self.category
                    ),
                    Err(error) => panic!("Err: {}", error),
                },
                Ok(false) => panic!("Err: Amount is not present in the total amount list"),
                Err(error) => panic!("Err: {}", error),
            },
            Ok(false) => panic!(
                "Err: {} category is not present in the total categories list",
                &self.category
            ),
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
                                let affected_rows = conn.execute(
                                    "delete from spend where category = ?",
                                    &[&self.category],
                                )?;

                                if affected_rows == 0 {
                                    return Err(rusqlite::Error::QueryReturnedNoRows);
                                }
                            }
                            Ok(false) => panic!(
                                "Err: {} category is not present in the budget list",
                                &self.category.as_str()
                            ),
                            Err(error) => panic!("Err: {}", error),
                        }
                    }
                    Ok(false) => panic!("Err: Amount is not present in the total amount list"),
                    Err(error) => panic!("Err: {}", error),
                }
            }
            Ok(false) => panic!(
                "Err: {} category is not present in the total categories list",
                &self.category.as_str()
            ),
            Err(error) => panic!("Err: {}", error),
        }

        Ok(())
    }
}
