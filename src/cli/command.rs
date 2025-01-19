use clap::Parser;
use crate::cli::subcommands::Command;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Move {
    #[clap(subcommand)]
    pub command: Command,
}