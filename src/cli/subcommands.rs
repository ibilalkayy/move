use crate::cli::flags::{
    AddTotal, RemoveTotal, UpdateTotal, ViewTotal, BlockchainInfo, BudgetData, CreateBudget, DBInfo, 
    GetBudget, GmailInfo, AlertData, UpdateBudget, AlertInfo, SpendData,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize your move application by inserting the database credentials.
    Init(InitInfo),

    /// Allows users to manage their budget allocations for different categories
    Budget(BudgetInfo),

    /// Provides spending services on various categories
    Spend(SpendInfo),

    /// Manage your total amount
    TotalAmount(TotalAmountInfo),
}

#[derive(Debug, Parser)]
pub struct InitInfo {
    #[clap(subcommand)]
    pub init_subcommand: InitSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum InitSubcommand {
    /// Insert the database credentials
    Database(DBInfo),

    /// Insert the blockchain credentials
    Blockchain(BlockchainInfo),

    /// Insert the gmail credentials
    Gmail(GmailInfo),
}

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
    Alert(BudgetAlert),
}

#[derive(Debug, Parser)]
pub struct BudgetAlert {
    #[clap(subcommand)]
    pub alert_subcommand: AlertSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AlertSubcommand {
    /// Set the alert notification data
    Set(AlertData),

    /// Get the alert notifications in your email
    Email(AlertInfo),

    /// See the alert notifications in your terminal
    See(AlertInfo),

    /// Update the alert values
    Update(AlertData),

    /// Remove the alert values
    Remove(AlertInfo),
}

#[derive(Debug, Parser)]
pub struct SpendInfo {
    #[clap(subcommand)]
    pub spend_subcommand: SpendSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SpendSubcommand {
    /// Data on which the money will be spent
    Money(SpendData),

    /// Shows the spending of various categories
    History,

    /// Removes the history data
    Remove,

    /// Show the history data
    Show,
}

#[derive(Debug, Parser)]
pub struct TotalAmountInfo {
    #[clap(subcommand)]
    pub total_amount: TotalAmountSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TotalAmountSubcommand {
    /// Add the total amount data
    Add(AddTotal),

    /// View the total amount data
    View(ViewTotal),

    /// Handle the total amount status
    Status(StatusTotal),

    /// Update the total amount data
    Update(UpdateTotal),

    /// Remove the total amount data
    Remove(RemoveTotal),
}

#[derive(Debug, Parser)]
pub struct StatusTotal {
    #[clap(subcommand)]
    pub status_subcommand: StatusSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum StatusSubcommand {
    /// Make the total amount active
    Active,

    /// Make the total amount inactive
    Inactive,

    /// Check the status of the total amount
    Check,
}
