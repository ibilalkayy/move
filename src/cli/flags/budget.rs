use clap::Parser;

#[derive(Debug, Parser)]
pub struct BudgetData {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: String,

    /// Total amount for the category
    #[clap(short, long)]
    pub amount: f64,
}

#[derive(Debug, Parser)]
pub struct BudgetCategory {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct UpdateBudget {
    /// Old category name to find
    #[clap(short, long)]
    pub old_category: String,

    /// New category name to update
    #[clap(short, long)]
    pub new_category: Option<String>,

    /// New amount of the category to update with
    #[clap(short, long)]
    pub amount: Option<f64>,
}
