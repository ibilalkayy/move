use crate::cli::subcommands::{
    budget::BudgetInfo, init::InitInfo, spend::SpendInfo, total_amount::TotalAmountInfo,
};

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize your move application by inserting the database credentials
    Init(InitInfo),

    /// Allocate the total amount to manage the budget and spending under the limit
    TotalAmount(TotalAmountInfo),

    /// Allows users to manage their budget allocations for different categories
    Budget(BudgetInfo),

    /// Provides spending services on various categories
    Spend(SpendInfo),
}
