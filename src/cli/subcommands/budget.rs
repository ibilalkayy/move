use crate::cli::flags::alert::{AlertData, AlertValues};
use crate::cli::flags::budget::{BudgetData, CreateBudget, GetBudget, UpdateBudget};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct BudgetInfo {
    #[clap(subcommand)]
    pub budget_subcommand: BudgetSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum BudgetSubcommand {
    /// Create the budget for different categories
    Create(CreateBudget),

    /// View the specific category details
    View(BudgetData),

    /// List all the budget details
    List,

    /// Get the budget data in a CSV file
    Get(GetBudget),

    /// Update the budget details
    Update(UpdateBudget),

    /// Delete the budget details
    Delete(BudgetData),

    /// Get the alert after passing the budget
    Alert(AlertBudget),
}

#[derive(Debug, Parser)]
pub struct AlertBudget {
    #[clap(subcommand)]
    pub alert_subcommand: AlertSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AlertSubcommand {
    /// Set the alert notification data
    Set(AlertData),

    /// See the alert notifications in your terminal
    See(AlertValues),

    /// Get the alert notifications in your email
    Email(AlertValues),

    /// Update the alert data
    Update(AlertData),

    /// Remove the alert data
    Remove(AlertValues),
}