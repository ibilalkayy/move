use crate::cli::flags::init::{BlockchainCred, DBCred, GmailCred};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct InitInfo {
    #[clap(subcommand)]
    pub init_subcommand: InitSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum InitSubcommand {
    /// Insert the database credentials
    Database(DBCred),

    /// Insert the blockchain credentials
    Blockchain(BlockchainCred),

    /// Insert the gmail credentials
    Gmail(GmailCred),
}
