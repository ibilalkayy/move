use crate::cli::flags::spend::{SpendCategory, SpendData};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct SpendInfo {
    #[clap(subcommand)]
    pub spend_subcommand: SpendSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SpendSubcommand {
    /// Spend the money on categories
    Money(SpendData),

    /// See the spending history
    History(SpendCategory),

    /// Delete the spending data
    Delete(SpendCategory),

    /// Get the spending data
    Get(GetData),
}

#[derive(Debug, Parser)]
pub struct GetData {
    #[clap(subcommand)]
    pub get_subcommand: GetSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GetSubcommand {
    /// Get the spending data by category
    Category(SpendCategory),

    /// Get all the spending data
    All,
}
