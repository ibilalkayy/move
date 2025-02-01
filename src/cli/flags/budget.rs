use clap::Parser;

#[derive(Debug, Parser)]
pub struct CreateBudget {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: String,

    /// Write the total amount for the category
    #[clap(short, long)]
    pub amount: String,
}

#[derive(Debug, Parser)]
pub struct BudgetData {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct GetBudget {
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

#[derive(Debug, Parser)]
pub struct UpdateBudget {
    /// Old category name to find
    #[clap(short, long)]
    pub old_category: String,

    /// New category name to update
    #[clap(short, long)]
    pub new_category: String,

    /// New amount of the category to update
    #[clap(short, long)]
    pub amount: String,
}
