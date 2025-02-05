use crate::cli::flags::cred::{BlockchainCred, GmailCred};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CredInfo {
    #[clap(subcommand)]
    pub cred_subcommand: CredSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CredSubcommand {
    /// Insert the blockchain credentials
    Blockchain(BlockchainCred),

    /// Insert the gmail credentials
    Gmail(GmailCred),
}
