use crate::cli::subcommands::total_amount::{
    AddTotalSubcommand, StatusSubcommand, TotalAmountInfo, TotalAmountSubcommand,
    UpdateTotalSubcommand, ViewSubcommand, RemoveTotalSubcommand,
};

use crate::data::handle_data::insert_data;

pub fn handle_total_amount(details: TotalAmountInfo) {
    match details.total_amount {
        TotalAmountSubcommand::Add(add_total) => match add_total.add_subcommand {
            AddTotalSubcommand::Amount(amount) => {
                // let _ = create_table();
                // let _ = amount.insert_total_amount();
                let header = ["Total Amount", "Spent Amount", "Remaining Amount", "Status"];
                let total_amount_detail = vec![vec![amount.amount, "0".to_string(), "0".to_string(), "inactive".to_string()]];
                insert_data(&header, total_amount_detail, "total_amount_data.csv", "Total amount").unwrap();
            }

            AddTotalSubcommand::Category(category) => {
                let header = ["Category", "Label"];
                let total_amount_detail = vec![vec![category.category, category.label]];
                insert_data(&header, total_amount_detail, "total_category_data.csv", "Total category").unwrap();
            }
        },

        TotalAmountSubcommand::View(view_total) => match view_total.view_subcommand {
            ViewSubcommand::Amount => {
                println!("view total amount");
            }

            ViewSubcommand::Categories => {
                println!("view total categories");
            }
        },

        TotalAmountSubcommand::Status(status_total) => match status_total.status_subcommand {
            StatusSubcommand::Active => {
                println!("Total amount is activated");
            }

            StatusSubcommand::Inactive => {
                println!("Total amount is now inactive");
            }

            StatusSubcommand::Check => {
                println!("total amount status is checked out");
            }
        },

        TotalAmountSubcommand::Update(update) => match update.update_subcommand {
            UpdateTotalSubcommand::Amount(_update_total) => {
                println!("Total amount is successfully updated");
            }

            UpdateTotalSubcommand::Categories(_update_category) => {
                println!("Total amount is successfully updated");
            }
        },

        TotalAmountSubcommand::Remove(remove_total) => match remove_total.remove_subcommand {
            RemoveTotalSubcommand::Amount => {
                println!("Total amount is successfully removed");
            }

            RemoveTotalSubcommand::Category(_remove_category) => {
                println!("Total amount category is successfully removed");
            }
        }
    }
}
