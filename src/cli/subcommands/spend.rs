use clap::{Parser, Subcommand};
use crate::cli::flags::spend::{SpendCategory, SpendData};

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

    /// Delete the spend history
    Delete(SpendCategory),

    /// Get the spending data
    Get(SpendData),
}
