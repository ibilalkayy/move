use ethers::{
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest},
    middleware::SignerMiddleware,
    utils::parse_ether,
};
use std::sync::Arc;
use eyre::Result;

pub async fn http_provider(alchemy_url: String, private_key: String, recepient_address: String, amount: f64, chain_id: u64) -> Result<()> {
    // Step 1: Connect to the RPC
    let provider = match Provider::<Http>::try_from(alchemy_url) {
        Ok(p) => {
            println!("✅ Connected to RPC successfully!");
            Arc::new(p)
        }
        Err(e) => {
            println!("❌ Failed to connect to RPC: {:?}", e);
            return Err(e.into());
        }
    };

    // Step 2: Load Wallet
    let wallet = match private_key.parse::<LocalWallet>() {
        Ok(w) => {
            println!("✅ Private key parsed successfully!");
            Arc::new(SignerMiddleware::new(provider.clone(), w.with_chain_id(chain_id)))
        }
        Err(e) => {
            println!("❌ Invalid private key: {:?}", e);
            return Err(e.into());
        }
    };

    // Step 3: Define Recipient Address
    let recipient = match recepient_address.parse::<Address>() {
        Ok(r) => {
            println!("✅ Recipient address parsed successfully!");
            r
        }
        Err(e) => {
            println!("❌ Invalid recipient address: {:?}", e);
            return Err(e.into());
        }
    };

    // Step 4: Convert Amount to Wei
    let amount = match parse_ether(amount) {
        Ok(a) => {
            println!("✅ Amount converted to Wei: {:?}", a);
            a
        }
        Err(e) => {
            println!("❌ Invalid amount: {:?}", e);
            return Err(e.into());
        }
    };

    // Step 5: Get Gas Price
    let gas_price = match provider.get_gas_price().await {
        Ok(g) => {
            println!("✅ Gas price fetched: {:?}", g);
            g
        }
        Err(e) => {
            println!("❌ Failed to fetch gas price: {:?}", e);
            return Err(e.into());
        }
    };

    // Step 6: Create Transaction
    let tx = TransactionRequest::new()
        .from(wallet.address())
        .to(recipient)
        .value(amount)
        .gas_price(gas_price);

    // Step 7: Send Transaction
    let pending_tx = match wallet.send_transaction(tx, None).await {
        Ok(tx) => {
            println!("✅ Transaction sent!");
            tx
        }
        Err(e) => {
            println!("❌ Failed to send transaction: {:?}", e);
            return Err(e.into());
        }
    };

    // Step 8: Wait for Confirmation
    match pending_tx.await {
        Ok(Some(receipt)) => {
            println!("✅ Transaction confirmed! Tx hash: {:?}", receipt.transaction_hash);
        }
        Ok(None) => {
            println!("❌ Transaction is pending, but no receipt yet.");
        }
        Err(e) => {
            println!("❌ Failed to get transaction receipt: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
