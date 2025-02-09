use clap::Parser;

#[derive(Debug, Parser)]
pub struct SpendData {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: Option<String>,

    /// Write the spending amount for a category
    #[clap(short, long)]
    pub amount: Option<String>,
}

#[derive(Debug, Parser)]
pub struct SpendFinder {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: String,
}