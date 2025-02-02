use crate::cli::flags::spend::SpendData;
use clap::{Parser, Subcommand};

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
