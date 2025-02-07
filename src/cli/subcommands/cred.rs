use crate::cli::flags::cred::{BlockchainCred, GmailCred, GetCred};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CredInfo {
    #[clap(subcommand)]
    pub cred_subcommand: CredSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CredSubcommand {
    /// Add the blockchain or gmail credentials
    Add(AddCred),

    /// View the blockchain or gmail credentials
    View(ViewCred),

    /// Update the blockchain or gmail credentials
    Update(UpdateCred),

    /// Delete the blockchain or gmail credentials
    Delete(DeleteCred),

    /// Get the blockchain data in a CSV file
    GetBlockchain(GetCred),

    /// Get the gmail data in a CSV file
    GetGmail(GetCred),
}

#[derive(Debug, Parser)]
pub struct AddCred {
    #[clap(subcommand)]
    pub add_subcommand: AddSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AddSubcommand {
    /// Insert the blockchain credentials
    Blockchain(BlockchainCred),

    /// Insert the gmail credentials
    Gmail(GmailCred),
}

#[derive(Debug, Parser)]
pub struct ViewCred {
    #[clap(subcommand)]
    pub view_subcommand: ViewSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ViewSubcommand {
    /// View the blockchain credentials
    Blockchain,

    /// View the gmail credentials
    Gmail,
}

#[derive(Debug, Parser)]
pub struct UpdateCred {
    #[clap(subcommand)]
    pub update_subcommand: UpdateSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UpdateSubcommand {
    /// Update the blockchain credentials
    Blockchain(BlockchainCred),

    /// Update the gmail credentials
    Gmail(GmailCred),
}

#[derive(Debug, Parser)]
pub struct DeleteCred {
    #[clap(subcommand)]
    pub delete_subcommand: DeleteSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DeleteSubcommand {
    /// Delete the blockchain credentials
    Blockchain,

    /// Delete the gmail credentials
    Gmail,
}
