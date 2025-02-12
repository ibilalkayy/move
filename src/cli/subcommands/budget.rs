use crate::cli::flags::budget::{BudgetCategory, BudgetData, UpdateBudget};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct BudgetInfo {
    #[clap(subcommand)]
    pub budget_subcommand: BudgetSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum BudgetSubcommand {
    /// Add the budget for different categories
    Add(BudgetData),

    /// View a specific category budget detail
    View(BudgetCategory),

    /// Show all the budget details
    Show,

    /// Get the budget data in a CSV file
    Get(BudgetData),

    /// Update the budget details
    Update(UpdateBudget),

    /// Delete the budget details
    Delete(BudgetCategory),
}