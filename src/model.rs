#[derive(Debug)]
pub struct TokenInfo {
    pub mint_time: String,
    pub mint_id: String,
    pub token_name: String,
    pub token_symbol: String,
}

impl TokenInfo {
    pub fn new(mint_time: &str, mint_id: &str, token_name: &str, token_symbol: &str) -> TokenInfo {
        let token_info = TokenInfo {
            mint_time: mint_time.to_string(),
            mint_id: mint_id.to_string(),
            token_name: token_name.to_string(),
            token_symbol: token_symbol.to_string(),
        };
        return token_info;
    }
}
