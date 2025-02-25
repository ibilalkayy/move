use crate::cli::subcommands::subcommands::Command;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    author,
    about = "Move is a budget planning application that will manage the budget based on different categories 
like (grocery, gadgets, etc). All the data will be stored in the move directory that is present in a home directory.",
    version
)]
pub struct Move {
    #[clap(subcommand)]
    pub command: Command,
}
