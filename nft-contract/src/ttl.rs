use crate::*;

#[near_bindgen]
impl Contract {

    pub fn nft_token_expires(&self, token_id: String) -> u64 {
        let expires = if let Some(token_metadata) = self.token_metadata_by_id.get(&token_id) {
            token_metadata.expires_at.unwrap_or(0)
        } else {
            0
        };
        expires
    }

    pub fn is_token_expires(&self, token_id: &TokenId) -> bool {
        let token_option = self.token_metadata_by_id.get(&token_id);
        match token_option {
            Some(token) => {
                is_expires(&token.expires_at)
            }
            None => true,
        }
        
    }

    #[payable]
    pub fn extend_token(&mut self, token_id: String) -> (u64, u128) {
        let deposit_balance = env::attached_deposit();
        let account_id = env::predecessor_account_id();
        assert!(deposit_balance >= self.price_per_year, "EXTEND_AT_LEAST_ONE_YEAR");
        let extend_years = u64::try_from(deposit_balance / self.price_per_year).ok().unwrap();
        let change = deposit_balance % self.price_per_year;
        let expires_at = self.internal_extend_token(&token_id, extend_years * ONE_YEAR_NANOSECOND);
        Promise::new(account_id).transfer(change);
        (expires_at, change)
    }
}

impl Contract {
    pub(crate) fn assert_token_expires(&self, token_id: &TokenId) {
        // check expires
        let token_metadata = self.token_metadata_by_id.get(token_id).expect("TOKEN_NOT_FOUND");
        let token_expires = token_metadata.expires_at;
        assert_expires(&token_expires);
    }

    pub(crate) fn internal_extend_token(&mut self, token_id: &TokenId, extend_ttl: u64) -> u64 {
        let current_block_timestamp = env::block_timestamp();
        let token_metadata = self.token_metadata_by_id.get(token_id).expect("NFT_NOT_FOUND");
        let new_expires_date = Some(token_metadata.expires_at.unwrap_or(current_block_timestamp) + extend_ttl);
        let update = Some(current_block_timestamp);
        let new_metadata = TokenMetadata {
            expires_at: new_expires_date,
            updated_at: update,
            ..token_metadata
        };
        self.token_metadata_by_id.insert(token_id, &new_metadata);
        new_expires_date.unwrap()
    }
}