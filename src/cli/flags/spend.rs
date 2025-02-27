use clap::Parser;

#[derive(Debug, Parser)]
pub struct SpendData {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: Option<String>,

    /// Write the spending amount for a category
    #[clap(short, long)]
    pub amount: Option<f64>,

    /// Write the recepient address
    #[clap(short, long)]
    pub recepient_address: Option<String>,
}

#[derive(Debug, Parser)]
pub struct SpendCategory {
    /// Write the category to perform the view, get, or delete action
    #[clap(short, long)]
    pub category: String,
}
