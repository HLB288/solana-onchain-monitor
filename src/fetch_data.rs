use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction_status::UiTransactionEncoding;

pub async fn get_balance(address: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let pubkey = address.parse::<Pubkey>()?;
    let balance = client.get_balance(&pubkey)?;
    println!("balance du wallet: {}", balance as f64 / 1_000_000.0);
    Ok(balance)
}

pub async fn get_transaction(address: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let pubkey = address.parse::<Pubkey>()?;
    let signature = client.get_signatures_for_address(&pubkey)?;
    let mut results = Vec::new();
    for sig in signature {
        let signature = sig.signature.parse::<Signature>()?;
        let tx = client.get_transaction(&signature, UiTransactionEncoding::Json)?;
        results.push(format!("{:?}", tx));
    }
    Ok(results)
}
