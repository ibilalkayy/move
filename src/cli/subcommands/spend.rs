use crate::cli::flags::spend::{SpendData, SpendFinder, GetSpending};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct SpendInfo {
    #[clap(subcommand)]
    pub spend_subcommand: SpendSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SpendSubcommand {
    /// Where the money will be spent
    Money(SpendData),

    /// Shows the spending based on various categories
    History(SpendFinder),

    /// Delete the spend history data
    Delete(SpendFinder),

    /// Show the history data
    Show,

    /// Get the spending data
    Get(GetSpending),
}
