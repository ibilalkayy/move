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
