use clap::Parser;

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
