use clap::Parser;
use crate::cli::command::Move;

pub fn cli() {
    let move_var = Move::parse();
    println!("{:#?}", move_var);
}