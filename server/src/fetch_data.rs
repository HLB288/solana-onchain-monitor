use crate::error::MonitorError;
use crate::model::TokenInfo;
use chrono::{DateTime, Utc};
use mpl_token_metadata::accounts::Metadata;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_client::rpc_response::OptionSerializer;
use solana_pubkey::Pubkey;
use solana_signature::Signature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status::UiTransactionEncoding;

pub async fn get_balance(address: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let pubkey = address.parse::<Pubkey>()?;
    let balance = client.get_balance(&pubkey)?;
    println!("balance du wallet: {}", balance as f64 / 1_000_000_000.0);
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

pub async fn get_transaction_by_signature(
    address: &str,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, MonitorError> {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com");
    let signature = address.parse::<Signature>()?;
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: None,
        max_supported_transaction_version: Some(0),
    };
    let tx = client.get_transaction_with_config(&signature, config)?;

    if let Some(time) = tx.block_time {
        let datetime = DateTime::<Utc>::from_timestamp(time, 0).unwrap();
        if let Some(ref meta) = tx.transaction.meta {
            if let OptionSerializer::Some(balances) = &meta.post_token_balances {
                if let Some(first) = balances
                    .iter()
                    .find(|b| b.mint != "So11111111111111111111111111111111111111112")
                {
                    let mint_pubkey = first.mint.parse::<Pubkey>()?;
                    let metadata_program_id =
                        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s".parse::<Pubkey>()?;
                    let seeds = &[
                        b"metadata" as &[u8],
                        metadata_program_id.as_ref(),
                        mint_pubkey.as_ref(),
                    ];
                    let (pda, _bump) = Pubkey::find_program_address(seeds, &metadata_program_id);
                    let pda_bytes = match client.get_account_data(&pda) {
                        Ok(data) => data,
                        Err(_) => {
                            println!("sans métadonnées");
                            return Ok(tx);
                        }
                    };
                    let pda_name = match Metadata::from_bytes(&pda_bytes) {
                        Ok(metadata) => metadata,
                        Err(_) => {
                            println!("sans métadonnées");
                            return Ok(tx);
                        }
                    };
                    let token_info = TokenInfo::new(
                        &datetime.to_string(),
                        &first.mint,
                        &pda_name.name.trim_matches('\0'),
                        &pda_name.symbol.trim_matches('\0'),
                    );
                    println!("{:?}", token_info);
                }
            }
        }
    }

    Ok(tx)
}
