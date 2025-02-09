use crate::cli::flags::spend::{SpendData, SpendFinder};
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

    /// Shows the spending history
    History(SpendFinder),

    /// Delete the spend history data
    Delete(SpendFinder),

    /// Get the spending data
    Get(SpendData),
}
