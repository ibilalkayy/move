use crate::cli::command::command::Move;
use crate::cli::subcommands::subcommands::Command;

use crate::cli::subcommands::{
    init::InitSubcommand,
    total_amount::{AddTotalSubcommand, StatusSubcommand, TotalAmountSubcommand, UpdateTotalSubcommand, ViewSubcommand},
    budget::{AlertSubcommand, BudgetSubcommand},
    spend::SpendSubcommand,
};

use crate::database::{
    total_amount::view_total_amount,
    total_categories::view_total_categories,
    budget::list_data,
};

use crate::database::db::create_table;
use clap::Parser;
use std::{fs::OpenOptions, io::Write};

fn file_options(data_detail: String) {
    let file_path = ".env";
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .expect("failed to open a file");

    file.write_all(data_detail.as_bytes())
        .expect("failed to write to file");
}

pub fn cli() {
    let moves = Move::parse();
    match moves.command {
        Command::Init(details) => match details.init_subcommand {
            InitSubcommand::Database(db_info) => {
                let data_detail = format!(
                    "\nDATABASE_URL=postgres://{}:{}@{}:{}/{}",
                    db_info.db_username, db_info.password, db_info.host, db_info.port, db_info.name,
                );

                file_options(data_detail);
                println!("Database data is successfully inserted");
            }

            InitSubcommand::Blockchain(b_info) => {
                let data_detail = format!(
                    "\nPRIVATE_KEY={}\nALCHEMY_URL={}\n",
                    b_info.private_key, b_info.alchemy_url,
                );

                file_options(data_detail);
                println!("Blockchain data is successfully inserted");
            }

            InitSubcommand::Gmail(g_info) => {
                let data_detail = format!(
                    "\nGMAIL_USERNAME={}\nGMAIL_ADDRESS={}\nGMAIL_APP_PASSWORD={}",
                    g_info.username, g_info.gmail, g_info.app_password,
                );

                file_options(data_detail);
                println!("Gmail data is successfully inserted");
            }
        },

        Command::TotalAmount(details) => match details.total_amount {
            TotalAmountSubcommand::Add(add_total) => match add_total.add_subcommand {
                AddTotalSubcommand::Amount(amount) => {
                    let _ = create_table();
                    let _ = amount.insert_total_amount();
                }

                AddTotalSubcommand::Categories(categories) => {
                    let _ = create_table();
                    let result = categories.insert_total_categories();
                    match result {
                        Ok(_) => println!("Total amount category is successfully saved"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
            },

            TotalAmountSubcommand::View(view_total) => match view_total.view_subcommand {
                ViewSubcommand::Amount => {
                    let _ = view_total_amount();
                }

                ViewSubcommand::Categories => {
                    let _ = view_total_categories();
                }
            },

            TotalAmountSubcommand::Status(status_total) => match status_total.status_subcommand {
                StatusSubcommand::Active => {
                    let result = status_total.update_status("active".to_string());
                    match result {
                        Ok(_) => println!("Total amount is activated"),
                        Err(err) => println!("Error: {}", err),
                    }
                }

                StatusSubcommand::Inactive => {
                    let result = status_total.update_status("inactive".to_string());
                    match result {
                        Ok(_) => println!("Total amount is now inactive"),
                        Err(err) => println!("Error: {}", err),
                    }
                }

                StatusSubcommand::Check => {
                    let _ = status_total.check_status();
                }
            },

            TotalAmountSubcommand::Update(update) => match update.update_subcommand {
                UpdateTotalSubcommand::Amount(update_total) => {
                    let result = update_total.update_total();
                    match result {
                        Ok(_) => println!("Total amount is successfully updated"),
                        Err(err) => println!("Error: {}", err),
                    }
                }

                UpdateTotalSubcommand::Categories(update_category) => {
                    let result = update_category.update_category();
                    match result {
                        Ok(_) => println!("Total amount is successfully updated"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
            },

            TotalAmountSubcommand::Remove(remove_total) => {
                let result = remove_total.remove_total();
                match result {
                    Ok(_) => println!("Alert data is successfully removed"),
                    Err(err) => println!("Error: {}", err),
                }
            }
        },

        Command::Budget(details) => match details.budget_subcommand {
            BudgetSubcommand::Create(create_budget) => {
                let _ = create_table();
                let result = create_budget.insert_data();
                match result {
                    Ok(_) => println!("Budget is successfully created"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            BudgetSubcommand::View(budget_data) => {
                let _ = budget_data.view_data();
            }

            BudgetSubcommand::List => {
                let _ = list_data();
            }

            BudgetSubcommand::Get(get_budget) => {
                let _ = get_budget.get_data();
            }

            BudgetSubcommand::Update(update_budget) => {
                let result = update_budget.update_data();
                match result {
                    Ok(_) => println!("Budget is successfully updated"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            BudgetSubcommand::Delete(budget_data) => {
                let result = budget_data.delete_data();
                match result {
                    Ok(category) => {
                        println!("{} category data has been successfully delete", category)
                    }
                    Err(err) => println!("err: {}", err),
                }
            }

            BudgetSubcommand::Alert(alert_budget) => match alert_budget.alert_subcommand {
                AlertSubcommand::Set(alert_data) => {
                    let _ = create_table();
                    let result = alert_data.create_alert();
                    match result {
                        Ok(_) => println!("Alert is successfully created"),
                        Err(err) => println!("Error: {}", err),
                    }
                }

                AlertSubcommand::Email(email_alert) => {
                    email_alert.get_alert();
                }

                AlertSubcommand::See(cli_alert) => {
                    cli_alert.see_alert();
                }

                AlertSubcommand::Update(update_alert) => {
                    let result = update_alert.update_data();
                    match result {
                        Ok(_) => println!("Alert is successfully updated"),
                        Err(err) => println!("Error: {}", err),
                    }
                }

                AlertSubcommand::Remove(remove_alert) => {
                    let result = remove_alert.remove_alert();
                    match result {
                        Ok(_) => println!("Alert data is successfully removed"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
            },
        },

        Command::Spend(details) => match details.spend_subcommand {
            SpendSubcommand::Money(spend_data) => {
                let _ = create_table();
                let result = spend_data.insert_spending();
                match result {
                    Ok(_) => println!("Spending data is successfully saved"),
                    Err(err) => println!("Error: {}", err),
                }
            }

            SpendSubcommand::History => {
                println!("history command");
            }

            SpendSubcommand::Remove => {
                println!("remove command");
            }

            SpendSubcommand::Show => {
                println!("show command");
            }
        },
    }
}
