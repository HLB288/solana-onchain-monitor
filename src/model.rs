pub struct TokenInfo {
    pub mint_time: String, 
    pub mint_id: String, 
    pub token_name: String,
}

impl TokenInfo {
    fn new (mint_time: &str, mint_id: &str, token_name: &str) -> TokenInfo {
        let token_info = TokenInfo {
            mint_time: mint_time.to_string(),
            mint_id: mint_id.to_string(),
            token_name: token_name.to_string(),
        };
        return token_info
    } 
}
