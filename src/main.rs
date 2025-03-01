mod cli;
mod common;
mod database;
mod middleware;
mod usecases;

use crate::cli::controller::cli::cli;

#[tokio::main]
async fn main() {
    cli().await;
}
