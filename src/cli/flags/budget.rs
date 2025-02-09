use clap::Parser;

#[derive(Debug, Parser)]
pub struct CreateBudget {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: Option<String>,

    /// Write the total amount for the category
    #[clap(short, long)]
    pub amount: Option<String>,
}

#[derive(Debug, Parser)]
pub struct BudgetData {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct UpdateBudget {
    /// Old category name to find
    #[clap(short, long)]
    pub old_category: Option<String>,

    /// New category name to update
    #[clap(short, long)]
    pub new_category: Option<String>,

    /// New amount of the category to update
    #[clap(short, long)]
    pub amount: Option<String>,
}
