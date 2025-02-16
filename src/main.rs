mod cli;
mod common;
mod database;
mod usecases;

use crate::cli::controller::cli::cli;

fn main() {
    cli();
}
