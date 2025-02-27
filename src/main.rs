mod cli;
mod common;
mod database;
mod usecases;
mod middleware;

use crate::cli::controller::cli::cli;

fn main() {
    cli();
}
