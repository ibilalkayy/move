use clap::{Parser, Subcommand};

use crate::cli::flags::{
    total_amount::{TotalAmount, UpdateTotalAmount},
    total_categories::{RemoveTotalCategory, TotalCategory, UpdateTotalCategory},
};

#[derive(Debug, Parser)]
pub struct TotalAmountInfo {
    #[clap(subcommand)]
    pub total_amount_subcommand: TotalAmountSubcommand,
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

    /// Get the total amount in a CSV file
    Get(GetTotal),
}

#[derive(Debug, Parser)]
pub struct AddTotal {
    #[clap(subcommand)]
    pub add_total: AddTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AddTotalSubcommand {
    /// Add the total amount
    Amount(TotalAmount),

    /// Add a category
    Category(TotalCategory),
}

#[derive(Debug, Parser)]
pub struct ViewTotal {
    #[clap(subcommand)]
    pub view_total: ViewSubcommand,
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
    pub status_total: StatusSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum StatusSubcommand {
    /// Make the total amount active
    Active,

    /// Make the total amount inactive
    Inactive,
}

#[derive(Debug, Parser)]
pub struct UpdateTotal {
    #[clap(subcommand)]
    pub update_total: UpdateTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UpdateTotalSubcommand {
    /// Update the total amount
    Amount(UpdateTotalAmount),

    /// Update the categories of total amount
    Categories(UpdateTotalCategory),
}

#[derive(Debug, Parser)]
pub struct RemoveTotal {
    #[clap(subcommand)]
    pub remove_total: RemoveTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum RemoveTotalSubcommand {
    /// Remove the total amount
    Amount,

    /// Remove a specific category
    Category(RemoveTotalCategory),
}

#[derive(Debug, Parser)]
pub struct GetTotal {
    #[clap(subcommand)]
    pub get_total: GetTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GetTotalSubcommand {
    /// Get the total amount data
    Amount(TotalAmount),

    /// Get the categories data
    Category(TotalCategory),
}
