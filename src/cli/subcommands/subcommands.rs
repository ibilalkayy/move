use clap::Subcommand;

use crate::cli::subcommands::{
    budget::BudgetInfo, cred::CredInfo,
    spend::SpendInfo, total_amount::TotalAmountInfo,
};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Insert the blockchain and gmail credentials
    Cred(CredInfo),

    /// Allocate the total amount to manage the budget and spending under the limit
    TotalAmount(TotalAmountInfo),

    /// Allows users to manage their budget allocations for different categories
    Budget(BudgetInfo),

    /// Provides spending services on various categories
    Spend(SpendInfo),
}
