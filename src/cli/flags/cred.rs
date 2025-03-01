use clap::Parser;

#[derive(Debug, Parser)]
pub struct BlockchainCred {
    /// Your Blockchain wallet private key
    #[clap(short = 'p', long, help = "Blockchain wallet private key")]
    pub private_key: String,

    /// Your Alchemy App URL for transaction
    #[clap(short = 'a', long, help = "Your Alchemy App URL")]
    pub alchemy_url: Option<String>,

    /// Chain ID of the network
    #[clap(short = 'c', long, help = "Chain ID of the network")]
    pub chain_id: u64,
}
