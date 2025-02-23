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
    Get(SpendCategory),
}
