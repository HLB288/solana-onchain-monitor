use test_fetch_solana_data::model::TokenInfo;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_info_new() {
        let token_info = TokenInfo::new(
            "2026-02-26 14:16:39 UTC",
            "12S64cvWPsF7MgpQN1CCtt78JYRZRTz32KFDDn1Srtsf",
            "TestToken",
            "TST",
        );
        assert_eq!(token_info.token_name, "TestToken");
        assert_eq!(token_info.mint_time, "2026-02-26 14:16:39 UTC");
        assert_eq!(
            token_info.mint_id,
            "12S64cvWPsF7MgpQN1CCtt78JYRZRTz32KFDDn1Srtsf"
        );
        assert_eq!(token_info.token_symbol, "TST");
    }
}
