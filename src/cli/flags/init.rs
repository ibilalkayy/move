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
