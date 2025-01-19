use clap::{Parser, Subcommand};

// The main command
#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Cash {
    #[clap(subcommand)]
    command: Command,
}

// All the subcommands
#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize your flow application before running other commands.
    Init(InitInfo),

    /// Allows users to manage their budget allocations for different spending categories
    Budget(BudgetInfo),

    /// Provides spending services on various categories
    Spend(SpendInfo),

    /// Manage your total amount
    TotalAmount(TotalAmountInfo),
}

// First subcommand flags
#[derive(Debug, Parser)]
struct InitInfo {
    /// Write the app password of your gmail account
    #[clap(short = 'a', long, help = "Your PostgreSQL app password")]
    app_password: String,

    /// The PostgreSQL DB name
    #[clap(short = 'n', long, help = "Your PostgreSQL database name")]
    name: String,

    /// Your Gmail address for alert notifications
    #[clap(short = 'g', long, help = "Your Gmail address")]
    gmail: String,

    /// The PostgreSQL host
    #[clap(short = 'o', long, help = "Your PostgreSQL host")]
    host: String,

    /// The PostgreSQL password
    #[clap(short = 'w', long, help = "Your PostgreSQL password")]
    password: String,

    /// The PostgreSQL port
    #[clap(short = 'p', long, help = "Your PostgreSQL port number")]
    port: u32,

    /// The PostgreSQL SSLMode
    #[clap(short = 's', long, help = "Your PostgreSQL database SSLMode")]
    sslmode: bool,

    /// The PostgreSQL username
    #[clap(short = 'u', long, help = "Your PostgreSQL database username")]
    db_username: String,

    /// Your username
    #[clap(short = 'e', long, help = "Your username")]
    username: String,
}

// Second subcommand
#[derive(Debug, Parser)]
struct BudgetInfo {
    #[clap(subcommand)]
    budget_subcommand: BudgetSubcommand,
}

// Second subcommand subcommands
#[derive(Debug, Subcommand)]
enum BudgetSubcommand {
    /// Create the budget for different categories
    Create(CreateInfo),

    /// View the budget details
    View(BudgetViewInfo),

    /// Get the budget data in CSV
    Get(GetInfo),

    /// Update the budget details
    Update(BudgetUpdateInfo),

    /// Delete the budget details
    Delete(DeleteInfo),

    /// Get the alert after passing the budget
    Alert(AlertInfo),
}

// Second subcommand subcommand flags
#[derive(Debug, Parser)]
struct CreateInfo {
    /// Write the Total amount for the category
    #[clap(short, long)]
    amount: u64,

    /// Category name (e.g., groceries, utilities, etc.)
    #[clap(short, long)]
    category: String,
}

#[derive(Debug, Parser)]
struct BudgetViewInfo {
    /// Category name to show specific details
    #[clap(short, long)]
    category: String,
}

#[derive(Debug, Parser)]
struct GetInfo {
    /// CSV file name where the data will be stored
    #[clap(short = 'n', long, help = "CSV file name where the data will be stored")]
    filename: String,

    /// File path to store the data in
    #[clap(short = 'p', long, help = "Filepath to store the data in")]
    filepath: String,
}

#[derive(Debug, Parser)]
struct BudgetUpdateInfo {
    /// New amount of the category to update
    #[clap(short, long)]
    amount: u64,

    /// New category name to allocate
    #[clap(short, long)]
    new_category: String,

    /// Old category name to update
    #[clap(short, long)]
    old_category: String,
}

#[derive(Debug, Parser)]
struct DeleteInfo {
    /// Category name to delete
    #[clap(short, long)]
    category: String,
}

#[derive(Debug, Parser)]
struct AlertInfo {
    #[clap(subcommand)]
    alert_subcommand: AlertSubcommand,
}

#[derive(Debug, Subcommand)]
enum AlertSubcommand {
    /// Setup for alert notification
    Setup(SetupInfo),

    /// Get alert notifications in your email
    Message(MessageInfo),

    /// View the alert notifications
    View(ViewAlertInfo),

    /// Update the alert values for notification
    Update(UpdateAlertInfo),

    /// Remove the alert values
    Remove(RemoveAlertInfo),
}

#[derive(Debug, Parser)]
struct SetupInfo {
    /// Category name to take its budget amount
    #[clap(short = 'c', long, help = "Category name to take its budget amount")]
    category: String,

    /// Frequency of notifications (e.g., hourly, daily, weekly, monthly)
    #[clap(short = 'f', long, help = "Frequency of notifications (e.g., hourly, daily, weekly, monthly)")]
    frequency: String,

    /// Preferred method of notification [email or CLI] message
    #[clap(short = 't', long, help = "Preferred method of notification [email or CLI] message")]
    method: String,

    /// A day to set the notification
    #[clap(short = 'd', long, help = "A day to set the notification")]
    day: u8,

    ///  An Hour to set the notification
    #[clap(short = 'o', long, help = "An hour to set the notification")]
    hour: u8,

    /// The minute to set the notification
    #[clap(short = 'm', long, help = "The minute to set the notification")]
    minute: u8,

    /// The second to set the notification
    #[clap(short = 's', long, help = "The second to set the notification")]
    second: u8,

    /// Write a weekday to set the notification
    #[clap(short = 'w', long, help = "Write a weekday to set the notification")]
    weekday: String,
}

#[derive(Debug, Parser)]
struct MessageInfo {
    /// Write the category to get the notification
    #[clap(short, long)]
    category: String,
}

#[derive(Debug, Parser)]
struct ViewAlertInfo {
    /// Write the category to see the alert values
    #[clap(short, long)]
    category: String,
}

#[derive(Debug, Parser)]
struct UpdateAlertInfo {
    /// Category name to take its budget amount
    #[clap(short = 'c', long, help = "Category name to take its budget amount")]
    category: String,

    /// Frequency of notifications (e.g., hourly, daily, weekly, monthly)
    #[clap(short = 'f', long, help = "Frequency of notifications (e.g., hourly, daily, weekly, monthly)")]
    frequency: String,

    /// Preferred method of notification [email or CLI] message
    #[clap(short = 't', long, help = "Preferred method of notification [email or CLI] message")]
    method: String,

    /// A day to set the notification
    #[clap(short = 'd', long, help = "A day to set the notification")]
    day: u8,

    ///  An Hour to set the notification
    #[clap(short = 'o', long, help = " An hour to set the notification")]
    hour: u8,

    /// The minute to set the notification
    #[clap(short = 'm', long, help = "The minute to set the notification")]
    minute: u8,

    /// The second to set the notification
    #[clap(short = 's', long, help = "The second to set the notification")]
    second: u8,

    /// Write a weekday to set the notification
    #[clap(short = 'w', long, help = "Write a weekday to set the notification")]
    weekday: String,
}

#[derive(Debug, Parser)]
struct RemoveAlertInfo {
    /// Write the category to remove the alert values
    #[clap(short, long)]
    category: String,
}

// Third subcommand
#[derive(Debug, Parser)]
struct SpendInfo {
    #[clap(subcommand)]
    spend_subcommand: SpendSubcommand,
}

// Third subcommand subcommands
#[derive(Debug, Subcommand)]
enum SpendSubcommand {
    /// Provides spending services on various categories.
    History,

    /// Removes the history data.
    Remove,

    /// Show the history data.
    Show,
}

// Fourth subcommand
#[derive(Debug, Parser)]
struct TotalAmountInfo {
    #[clap(subcommand)]
    total_amount: TotalAmountSubcommand,
}

// Fourth subcommand subcommands
#[derive(Debug, Subcommand)]
enum TotalAmountSubcommand {
    /// Add the total amount data
    Add(AddInfo),

    /// View the total amount data
    View(ViewInfo),

    /// Handle the total amount status
    Status(StatusInfo),

    /// Update the total amount data
    Update(UpdateInfo),

    /// Remove the total amount data
    Remove(RemoveInfo),
}

// Fourth subcommand subcommand flags
#[derive(Debug, Parser)]
struct AddInfo {
    /// Write the total amount that you want to set
    #[clap(short, long)]
    amount: u64,

    /// Specify a category to include in the total amount
    #[clap(short, long)]
    category: String,
    
    /// Provide a label for setting up your total amount
    #[clap(short, long)]
    label: String,
}

#[derive(Debug, Parser)]
struct ViewInfo {
    /// View the total amount
    #[clap(short, long)]
    amount: u64,

    /// View the categories in total amount
    #[clap(short, long)]
    categories: String,
}

#[derive(Debug, Parser)]
struct StatusInfo {
    #[clap(subcommand)]
    status_subcommand: StatusSubcommand,
}

#[derive(Debug, Subcommand)]
enum StatusSubcommand {
    /// Make the total amount active
    Active,

    /// Make the total amount inactive
    Inactive,
    
    /// Check the status of the total amount
    Check,
}

#[derive(Debug, Parser)]
struct UpdateInfo {
    /// Write the total amount that you want to update
    #[clap(short, long)]
    amount: u64,

    /// Write the label that you want to update
    #[clap(short, long)]
    label: String,

    // Write the new category to update with
    #[clap(short, long)]
    new_category: String,

    // Write the old category to update
    #[clap(short, long)]
    old_category: String,
}

#[derive(Debug, Parser)]
struct RemoveInfo {
    /// Remove the total amount data
    #[clap(short, long)]
    category: String,
}

fn main() {
    let cash = Cash::parse();
    println!("{:#?}", cash);
}

