use clap::Parser;

#[derive(Debug, Parser)]
pub struct TotalAmount {
    /// Write the total amount to setup
    #[clap(short, long)]
    pub amount: Option<String>,
}

#[derive(Debug, Parser)]
pub struct TotalCategory {
    /// Write a category to include in the total amount
    #[clap(short, long)]
    pub category: Option<String>,

    /// Write a label to include in your total amount
    #[clap(short, long)]
    pub label: Option<String>,
}

#[derive(Debug, Parser)]
pub struct UpdateTotalAmount {
    /// Write the total amount to update
    #[clap(short, long)]
    pub amount: String,
}

#[derive(Debug, Parser)]
pub struct UpdateTotalCategory {
    // Write the new category to update with
    #[clap(short, long)]
    pub new_category: Option<String>,

    // Write the old category to update
    #[clap(short, long)]
    pub old_category: Option<String>,

    /// Write the label to update
    #[clap(short, long)]
    pub label: Option<String>,
}

#[derive(Debug, Parser)]
pub struct RemoveTotalCategory {
    /// Remove the total amount data
    #[clap(short, long)]
    pub category: String,
}