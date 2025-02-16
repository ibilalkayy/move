use crate::cli::flags::alert::{AlertCategory, AlertData};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct AlertInfo {
    #[clap(subcommand)]
    pub alert_subcommand: AlertSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AlertSubcommand {
    /// Add the alert notification data
    Add(AlertData),

    /// View the alert notification data
    View,

    /// See the alert notifications in your terminal
    See(AlertCategory),

    /// Get the alert notifications in your email
    Email(AlertCategory),

    /// Update the alert data
    Update(AlertData),

    /// Delete the alert data
    Delete(AlertCategory),

    /// Get the alert data in a CSV file
    Get(AlertData),
}
