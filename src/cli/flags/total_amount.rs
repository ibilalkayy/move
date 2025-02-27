use clap::Parser;

#[derive(Debug, Parser)]
pub struct TotalAmount {
    /// Write the total amount to add
    #[clap(short, long)]
    pub amount: Option<f64>,
}

#[derive(Debug, Parser)]
pub struct UpdateTotalAmount {
    /// Write the total amount to update
    #[clap(short, long)]
    pub amount: f64,
}
