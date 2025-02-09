use clap::Parser;

#[derive(Debug, Parser)]
pub struct BlockchainCred {
    /// Your Blockchain wallet private key
    #[clap(short = 'p', long, help = "Blockchain wallet private key")]
    pub private_key: Option<String>,

    /// Your Alchemy URL
    #[clap(short = 'a', long, help = "Your alchemy URL")]
    pub alchemy_url: Option<String>,
}

#[derive(Debug, Parser)]
pub struct GmailCred {
    /// Your username
    #[clap(short = 'u', long, help = "Your username")]
    pub username: Option<String>,

    /// Your gmail address
    #[clap(short = 'g', long, help = "Your gmail address")]
    pub gmail_address: Option<String>,

    /// Your gmail account's app password
    #[clap(short = 'a', long, help = "Your gmail's app password")]
    pub app_password: Option<String>,
}
