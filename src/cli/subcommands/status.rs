use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct StatusInfo {
    #[clap(subcommand)]
    pub status_info: StatusSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum StatusSubcommand {
    /// Make the total amount active
    Active,

    /// Make the total amount inactive
    Inactive,

    /// Check the status
    Check,
}
