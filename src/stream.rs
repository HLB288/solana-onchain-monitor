use crate::fetch_data::get_transaction_by_signature;
use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};

pub async fn fetch_token() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://api.mainnet-beta.solana.com";
    let filter = RpcTransactionLogsFilter::Mentions(vec![
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
    ]);
    let config = RpcTransactionLogsConfig { commitment: None };
    let (_subscription, receiver) = PubsubClient::logs_subscribe(url, filter, config)?;
    loop {
        // let log = receiver.recv()?;
        // println!("Nouveau log : {:?}," log);
        let log = receiver.recv()?;
        // if log.contains("InitializeMint") {
        // if log.value.signature.contains("InitializeMint") {
        if log
            .value
            .logs
            .iter()
            .any(|element| element.contains("InitializeMint"))
        {
            println!("=== Nouveau token détecté ===");
            let _tx = get_transaction_by_signature(&log.value.signature).await;
            // println!("Transaction: {:?}", tx);
            // println!("Signature {}", get_transaction_by_signature(&log.value.signature)).await?;
        }
    }
}
