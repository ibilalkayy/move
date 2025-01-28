use clap::Parser;

#[derive(Debug, Parser)]
pub struct DBCred {
    /// The PostgreSQL DB name
    #[clap(short = 'n', long, help = "Your PostgreSQL database name")]
    pub name: String,

    /// The PostgreSQL host
    #[clap(short = 'o', long, help = "Your PostgreSQL host")]
    pub host: String,

    /// The PostgreSQL password
    #[clap(short = 'a', long, help = "Your PostgreSQL password")]
    pub password: String,

    /// The PostgreSQL port
    #[clap(short = 'p', long, help = "Your PostgreSQL port number")]
    pub port: String,

    /// The PostgreSQL username
    #[clap(short = 'u', long, help = "Your PostgreSQL username")]
    pub db_username: String,
}

#[derive(Debug, Parser)]
pub struct BlockchainCred {
    /// Your Blockchain wallet private key
    #[clap(short = 'p', long, help = "Blockchain wallet private key")]
    pub private_key: String,

    /// Your Alchemy URL
    #[clap(short = 'a', long, help = "Your alchemy URL")]
    pub alchemy_url: String,
}

#[derive(Debug, Parser)]
pub struct GmailCred {
    /// Your username
    #[clap(short = 'u', long, help = "Your username")]
    pub username: String,

    /// Your Gmail address for alert notifications
    #[clap(short = 'g', long, help = "Your Gmail address")]
    pub gmail: String,

    /// Write the app password of your gmail account
    #[clap(short = 'a', long, help = "Your PostgreSQL app password")]
    pub app_password: String,
}

#[derive(Debug, Parser)]
pub struct AddTotalAmount {
    /// Write the total amount to setup
    #[clap(short, long)]
    pub amount: String,
}

#[derive(Debug, Parser)]
pub struct AddTotalCategories {
    /// Write a category to include in the total amount
    #[clap(short, long)]
    pub category: String,

    /// Write a label to include in your total amount
    #[clap(short, long)]
    pub label: String,
}

#[derive(Debug, Parser)]
pub struct UpdateTotal {
    // Write the new category to update with
    #[clap(short, long)]
    pub new_category: String,

    // Write the old category to update
    #[clap(short, long)]
    pub old_category: String,

    /// Write the total amount that you want to update
    #[clap(short, long)]
    pub amount: String,

    /// Write the label that you want to update
    #[clap(short, long)]
    pub label: String,
}

#[derive(Debug, Parser)]
pub struct RemoveTotal {
    /// Remove the total amount data
    #[clap(short, long)]
    pub category: String,
}

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

#[derive(Debug, Parser)]
pub struct AlertData {
    /// Category name to create new or update new
    #[clap(
        short = 'c',
        long,
        help = "Category name to create new or update new"
    )]
    pub category: String,

    /// Old category name to update it with a new one
    #[clap(
        short = 'l',
        long,
        help = "Old category name to update it with a new one"
    )]
    pub old_category: Option<String>,

    /// Frequency of notifications (e.g., hourly, daily, weekly, monthly)
    #[clap(
        short = 'f',
        long,
        help = "Frequency of notifications (e.g., hourly, daily, weekly, monthly)"
    )]
    pub frequency: String,

    /// Preferred method of notification [email or CLI] message
    #[clap(
        short = 't',
        long,
        help = "Preferred method of notification [email or CLI] message"
    )]
    pub method: String,

    /// A day for notification
    #[clap(
        short = 'd',
        long,
        help = "A day for notification"
    )]
    pub day: String,

    /// An Hour for notification
    #[clap(
        short = 'o',
        long,
        help = "An hour for notification"
    )]
    pub hour: String,

    /// A minute for notification
    #[clap(
        short = 'm',
        long,
        help = "A minute for notification"
    )]
    pub minute: String,

    /// A second for notification
    #[clap(
        short = 's',
        long,
        help = "A second for notification"
    )]
    pub second: String,

    /// Write a weekday for notification
    #[clap(
        short = 'w',
        long,
        help = "Write a weekday for notification"
    )]
    pub weekday: String,
}

#[derive(Debug, Parser)]
pub struct AlertValues {
    /// Write a category to see, get, or remove the alert notification
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct SpendData {
    /// Write the category name to spend the money on
    #[clap(short, long)]
    pub category: String,

    /// Write the spending amount for a category
    #[clap(short, long)]
    pub amount: String,
}