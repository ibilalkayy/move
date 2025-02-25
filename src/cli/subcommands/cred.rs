use crate::cli::flags::cred::BlockchainCred;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CredInfo {
    #[clap(subcommand)]
    pub cred_subcommand: CredSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CredSubcommand {
    /// Add the blockchain credentials
    Add(BlockchainCred),

    /// View the blockchaincredentials
    View,

    /// Update the blockchain credentials
    Update(BlockchainCred),

    /// Delete the blockchain credentials
    Delete,
}