use clap::Parser;

#[derive(Debug, Parser)]
pub struct AddTotalAmount {
    /// Write the total amount to setup
    #[clap(short, long)]
    pub amount: String,
}

#[derive(Debug, Parser)]
pub struct AddTotalCategory {
    /// Write a category to include in the total amount
    #[clap(short, long)]
    pub category: String,

    /// Write a label to include in your total amount
    #[clap(short, long)]
    pub label: String,
}

#[derive(Debug, Parser)]
pub struct UpdateTotalAmount {
    /// Write the total amount to update
    #[clap(short, long)]
    pub amount: String,
}

#[derive(Debug, Parser)]
pub struct UpdateTotalCategories {
    // Write the new category to update with
    #[clap(short, long)]
    pub new_category: String,

    // Write the old category to update
    #[clap(short, long)]
    pub old_category: String,

    /// Write the label that you want to update
    #[clap(short, long)]
    pub label: String,
}

#[derive(Debug, Parser)]
pub struct RemoveTotalCategory {
    /// Remove the total amount data
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct GetTotal {
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
