use clap::Parser;

#[derive(Debug, Parser)]
pub struct SpendData {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: String,

    /// Write the spending amount for a category
    #[clap(short, long)]
    pub amount: String,
}
