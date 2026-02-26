use thiserror::Error;

#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("RPC error: {0}")]
    RpcError(#[from] solana_client::client_error::ClientError), 
    #[error("Parsing Pubkey Error: {0}")]
    ParsePubkeyError(#[from] solana_pubkey::ParsePubkeyError),
    #[error("Parsing signature error {0}")]
    ParseSignatureError(#[from] solana_signature::ParseSignatureError),
}