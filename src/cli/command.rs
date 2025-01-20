use crate::cli::subcommands::Command;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Move {
    #[clap(subcommand)]
    pub command: Command,
}
