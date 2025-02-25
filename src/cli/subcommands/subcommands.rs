use clap::Subcommand;

use crate::cli::subcommands::{
    budget::BudgetInfo, cred::CredInfo, spend::SpendInfo, status::StatusInfo,
    total_amount::TotalAmountInfo,
};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Inserts the blockchain credentials and use them
    Cred(CredInfo),

    /// Allocates the total amount to manage the budget and spending under the limit
    TotalAmount(TotalAmountInfo),

    /// Allows users to handle their budget allocations for different categories
    Budget(BudgetInfo),

    /// Allows users to spend money on different ategories
    Spend(SpendInfo),

    /// Allows users to change the status of the spending
    Status(StatusInfo),
}
