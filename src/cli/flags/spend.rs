use clap::Parser;

#[derive(Debug, Parser)]
pub struct SpendData {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: Option<String>,

    /// Write the spending amount for a category
    #[clap(short, long)]
    pub amount: Option<f64>,

    /// Write the recepient blockchain address
    #[clap(short, long)]
    pub recepient_address: Option<String>,

    /// Write the private key, key
    #[clap(short, long)]
    pub private_key: Option<String>,

    /// Write the alchemy url key
    #[clap(short = 'k', long)]
    pub alchemy_url_key: Option<String>,
}

#[derive(Debug, Parser)]
pub struct SpendCategory {
    /// Write the category to perform the view, get, or delete action
    #[clap(short, long)]
    pub category: String,
}
