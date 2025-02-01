use crate::cli::flags::{
    DBCred, BlockchainCred, GmailCred, AddTotalAmount, AddTotalCategories, UpdateTotalAmount, UpdateTotalCategories, 
    RemoveTotal, BudgetData, CreateBudget, GetBudget, UpdateBudget, AlertData, AlertValues, SpendData,
};
use clap::{Parser, Subcommand};

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

#[derive(Debug, Parser)]
pub struct InitInfo {
    #[clap(subcommand)]
    pub init_subcommand: InitSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum InitSubcommand {
    /// Insert the database credentials
    Database(DBCred),

    /// Insert the blockchain credentials
    Blockchain(BlockchainCred),

    /// Insert the gmail credentials
    Gmail(GmailCred),
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
pub struct AddTotal {
    #[clap(subcommand)]
    pub add_subcommand: AddTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AddTotalSubcommand {
    /// Add the total amount
    Amount(AddTotalAmount),

    /// Add the categories
    Categories(AddTotalCategories),
}

#[derive(Debug, Parser)]
pub struct ViewTotal {
    #[clap(subcommand)]
    pub view_subcommand: ViewSubcommand,
}
 
#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    /// View the total amount
    Amount,

    /// View the categories of total amount
    Categories,
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

#[derive(Debug, Parser)]
pub struct UpdateTotal {
    #[clap(subcommand)]
    pub update_subcommand: UpdateTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UpdateTotalSubcommand {
    /// Update the total amount
    Amount(UpdateTotalAmount),

    /// Update the categories
    Categories(UpdateTotalCategories),
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