use clap::Parser;

#[derive(Debug, Parser)]
pub struct SpendData {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: Option<String>,

    /// Write the spending amount for a category
    #[clap(short, long)]
    pub amount: Option<u64>,
}

#[derive(Debug, Parser)]
pub struct SpendCategory {
    /// Write the category to perform the view, get, or delete action
    #[clap(short, long)]
    pub category: String,
}
