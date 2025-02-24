use clap::Subcommand;

use crate::cli::subcommands::{
    budget::BudgetInfo, cred::CredInfo, spend::SpendInfo,
    total_amount::TotalAmountInfo, status::StatusInfo,
};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Inserts the blockchain and gmail credentials and use them
    Cred(CredInfo),

    /// Allocates the total amount to manage the budget and spending under the limit
    TotalAmount(TotalAmountInfo),

    /// Allows users to handle their budget allocations for different categories
    Budget(BudgetInfo),

    /// Provides spending services on various categories
    Spend(SpendInfo),

    /// Allows users to handle the status of the spending
    Status(StatusInfo),
}
