use crate::cli::flags::total_amount::{
    AddTotalAmount, AddTotalCategory, RemoveTotalCategory, UpdateTotalAmount, UpdateTotalCategories, GetTotal,
};

use clap::{Parser, Subcommand};

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

    /// Get the total amount data in CSV file
    Get(GetTotal),
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

    /// Add a category
    Category(AddTotalCategory),
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
pub struct RemoveTotal {
    #[clap(subcommand)]
    pub remove_subcommand: RemoveTotalSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum RemoveTotalSubcommand {
    /// Remove the total amount
    Amount,

    /// Remove a specific category
    Category(RemoveTotalCategory),
}
