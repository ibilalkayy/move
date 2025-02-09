use clap::Parser;

#[derive(Debug, Parser)]
pub struct AlertData {
    /// Category name to create new or update new
    #[clap(short = 'c', long, help = "Category name to create new or update new")]
    pub category: Option<String>,

    /// Old category name to update it with a new one
    #[clap(
        short = 'l',
        long,
        help = "Old category name to update it with a new one"
    )]
    pub old_category: Option<String>,

    /// Frequency of notification (e.g., hourly, daily, weekly, monthly)
    #[clap(
        short = 'f',
        long,
        help = "Frequency of notifications (e.g., hourly, daily, weekly, monthly)"
    )]
    pub frequency: Option<String>,

    /// Preferred method for notification [email or CLI] message
    #[clap(
        short = 't',
        long,
        help = "Preferred method of notification [email or CLI] message"
    )]
    pub method: Option<String>,

    /// A day for notification
    #[clap(short = 'd', long, help = "A day for notification")]
    pub day: Option<String>,

    /// An Hour for notification
    #[clap(short = 'o', long, help = "An hour for notification")]
    pub hour: Option<String>,

    /// A minute for notification
    #[clap(short = 'm', long, help = "A minute for notification")]
    pub minute: Option<String>,

    /// A second for notification
    #[clap(short = 's', long, help = "A second for notification")]
    pub second: Option<String>,

    /// Write a weekday for notification
    #[clap(short = 'w', long, help = "A weekday for notification")]
    pub weekday: Option<String>,
}

#[derive(Debug, Parser)]
pub struct AlertCategory {
    /// A category to see, get, or remove the alert notification
    #[clap(short, long)]
    pub category: String,
}