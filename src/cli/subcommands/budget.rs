use crate::cli::flags::{
    alert::{AlertData, AlertCategory},
    budget::{BudgetCategory, BudgetData, UpdateBudget},
};
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

    /// Get the alert after passing the budget
    Alert(AlertBudget),
}

#[derive(Debug, Parser)]
pub struct AlertBudget {
    #[clap(subcommand)]
    pub alert_budget: AlertSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AlertSubcommand {
    /// Add the alert notification data
    Add(AlertData),

    /// View the alert notification data
    View,

    /// See the alert notifications in your terminal
    See(AlertCategory),

    /// Get the alert notifications in your email
    Email(AlertCategory),

    /// Update the alert data
    Update(AlertData),

    /// Delete the alert data
    Delete(AlertCategory),

    /// Get the alert data in a CSV file
    Get(AlertData),
}
