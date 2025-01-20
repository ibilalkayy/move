use crate::cli::command::Move;
use clap::Parser;

pub fn cli() {
    let move_var = Move::parse();
    println!("{:#?}", move_var);
}
