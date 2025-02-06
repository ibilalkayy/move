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

#[derive(Debug, Parser)]
pub struct SpendFinder {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct GetSpending {
    /// CSV file name where the data will be stored
    #[clap(
        short = 'n',
        long,
        help = "CSV file name where the data will be stored"
    )]
    pub filename: String,

    /// File path to store the data in
    #[clap(short = 'p', long, help = "Filepath to store the data in")]
    pub filepath: String,
}