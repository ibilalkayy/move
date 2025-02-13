mod cli;
mod database;
mod usecases;
mod common;

use crate::cli::controller::cli::cli;

fn main() {
    cli();
}
