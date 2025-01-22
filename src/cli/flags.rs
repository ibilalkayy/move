use clap::Parser;

#[derive(Debug, Parser)]
pub struct DBInfo {
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
pub struct BlockchainInfo {
    /// Your Blockchain wallet private key
    #[clap(short = 'p', long, help = "Blockchain wallet private key")]
    pub private_key: String,

    /// Your Alchemy URL
    #[clap(short = 'a', long, help = "Your alchemy URL")]
    pub alchemy_url: String,
}

#[derive(Debug, Parser)]
pub struct GmailInfo {
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
pub struct BudgetData {
    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    pub category: String,

    /// Write the total amount for the category
    #[clap(short, long)]
    pub amount: i64,
}

#[derive(Debug, Parser)]
pub struct GetInfo {
    /// CSV file name where the data will be stored
    #[clap(short = 'n', long, help = "CSV file name where the data will be stored")]
    pub filename: String,

    /// File path to store the data in
    #[clap(short = 'p', long, help = "Filepath to store the data in")]
    pub filepath: String,
}

#[derive(Debug, Parser)]
pub struct BudgetUpdateInfo {
    /// New category name to allocate
    #[clap(short, long)]
    pub new_category: String,

    /// Old category name to update
    #[clap(short, long)]
    pub old_category: String,

    /// New amount of the category to update
    #[clap(short, long)]
    pub amount: u64,
}

#[derive(Debug, Parser)]
pub struct DeleteInfo {
    /// Category name to delete
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct SetupInfo {
    /// Category name to take its budget amount
    #[clap(short = 'c', long, help = "Category name to take its budget amount")]
    pub category: String,

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

    /// A day to set the notification
    #[clap(short = 'd', long, help = "A day to set the notification")]
    pub day: u8,

    ///  An Hour to set the notification
    #[clap(short = 'o', long, help = "An hour to set the notification")]
    pub hour: u8,

    /// The minute to set the notification
    #[clap(short = 'm', long, help = "The minute to set the notification")]
    pub minute: u8,

    /// The second to set the notification
    #[clap(short = 's', long, help = "The second to set the notification")]
    pub second: u8,

    /// Write a weekday to set the notification
    #[clap(short = 'w', long, help = "Write a weekday to set the notification")]
    pub weekday: String,
}

#[derive(Debug, Parser)]
pub struct MessageInfo {
    /// Write the category to get the notification
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct ViewAlertInfo {
    /// Write the category to see the alert values
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct UpdateAlertInfo {
    /// Category name to take its budget amount
    #[clap(short = 'c', long, help = "Category name to take its budget amount")]
    pub category: String,

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

    /// A day to set the notification
    #[clap(short = 'd', long, help = "A day to set the notification")]
    pub day: u8,

    ///  An Hour to set the notification
    #[clap(short = 'o', long, help = " An hour to set the notification")]
    pub hour: u8,

    /// The minute to set the notification
    #[clap(short = 'm', long, help = "The minute to set the notification")]
    pub minute: u8,

    /// The second to set the notification
    #[clap(short = 's', long, help = "The second to set the notification")]
    pub second: u8,

    /// Write a weekday to set the notification
    #[clap(short = 'w', long, help = "Write a weekday to set the notification")]
    pub weekday: String,
}

#[derive(Debug, Parser)]
pub struct RemoveAlertInfo {
    /// Write the category to remove the alert values
    #[clap(short, long)]
    pub category: String,
}

#[derive(Debug, Parser)]
pub struct AddInfo {
    /// Specify a category to include in the total amount
    #[clap(short, long)]
    pub category: String,

    /// Write the total amount that you want to set
    #[clap(short, long)]
    pub amount: u64,

    /// Provide a label for setting up your total amount
    #[clap(short, long)]
    pub label: String,
}

#[derive(Debug, Parser)]
pub struct ViewInfo {
    /// View the categories in total amount
    #[clap(short, long)]
    pub categories: String,

    /// View the total amount
    #[clap(short, long)]
    pub amount: u64,
}

#[derive(Debug, Parser)]
pub struct UpdateInfo {
    // Write the new category to update with
    #[clap(short, long)]
    pub new_category: String,

    // Write the old category to update
    #[clap(short, long)]
    pub old_category: String,

    /// Write the total amount that you want to update
    #[clap(short, long)]
    pub amount: u64,

    /// Write the label that you want to update
    #[clap(short, long)]
    pub label: String,
}

#[derive(Debug, Parser)]
pub struct RemoveInfo {
    /// Remove the total amount data
    #[clap(short, long)]
    pub category: String,
}
