use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::RpcTransactionLogsFilter; 

pub async fn fetch_token() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://api.mainnet-beta.solana.com";
    let client = PubsubClient::new(url);
    let filter = RpcTransactionLogsFilter::Mention(vec![
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".to_string(),
    ]);
    let (subcription, receiver) = client.logs_subscribe(filter)?;
    loop {
        let log = receiver.recv()?;
        println!("Nouveau log : {:?}," log);
    }
}
