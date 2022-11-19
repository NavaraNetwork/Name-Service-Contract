use crate::*;

pub(crate) fn is_expires(expires_time_option: &Option<u64>) -> bool {
    let expires_time = expires_time_option.unwrap_or(0);
    expires_time < env::block_timestamp()
}

pub(crate) fn assert_expires(expires_time_option: &Option<u64>) {
    let expires_time = expires_time_option.unwrap_or(0);
    assert!(
        !is_expires(expires_time_option),
        "expires: current {} - expires_time {}",
        env::block_timestamp(),
        expires_time
    )
}



pub(crate) fn assert_owner(owner: &AccountId, account_id: &AccountId) {
    assert_eq!(
        owner, 
        account_id,
        "ONLY_CONTRACT_OWNER"
    )
}

impl Contract {
    pub(crate) fn assert_token_owner(&self, token_id: &TokenId) {
        let token = self.tokens_by_id.get(&token_id).expect("TOKEN_NOT_FOUND");
        assert_eq!(
            env::predecessor_account_id(), 
            token.owner_id,
            "ONLY_NFT_OWNER"
        );
    }

}