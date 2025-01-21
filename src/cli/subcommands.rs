use crate::cli::flags::{
    AddInfo, BudgetUpdateInfo, BudgetViewInfo, CreateInfo, DeleteInfo, GetInfo, DBInfo, BlockchainInfo, GmailInfo,
    MessageInfo, RemoveAlertInfo, RemoveInfo, SetupInfo, UpdateAlertInfo, UpdateInfo,
    ViewAlertInfo, ViewInfo,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize your move application by inserting the database credentials.
    Init(InitInfo),

    /// Allows users to manage their budget allocations for different spending categories
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
    Create(CreateInfo),

    /// View the budget details
    View(BudgetViewInfo),

    /// Get the budget data in CSV
    Get(GetInfo),

    /// Update the budget details
    Update(BudgetUpdateInfo),

    /// Delete the budget details
    Delete(DeleteInfo),

    /// Get the alert after passing the budget
    Alert(AlertInfo),
}

#[derive(Debug, Parser)]
pub struct AlertInfo {
    #[clap(subcommand)]
    pub alert_subcommand: AlertSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AlertSubcommand {
    /// Setup for alert notification
    Setup(SetupInfo),

    /// Get alert notifications in your email
    Message(MessageInfo),

    /// View the alert notifications
    View(ViewAlertInfo),

    /// Update the alert values for notification
    Update(UpdateAlertInfo),

    /// Remove the alert values
    Remove(RemoveAlertInfo),
}

#[derive(Debug, Parser)]
pub struct SpendInfo {
    #[clap(subcommand)]
    pub spend_subcommand: SpendSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SpendSubcommand {
    /// Provides spending services on various categories.
    History,

    /// Removes the history data.
    Remove,

    /// Show the history data.
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
    Add(AddInfo),

    /// View the total amount data
    View(ViewInfo),

    /// Handle the total amount status
    Status(StatusInfo),

    /// Update the total amount data
    Update(UpdateInfo),

    /// Remove the total amount data
    Remove(RemoveInfo),
}

#[derive(Debug, Parser)]
pub struct StatusInfo {
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
