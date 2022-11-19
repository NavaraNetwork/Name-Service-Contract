use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn storage_deposit(&mut self) -> (Balance, u128) {
        let deposit_amount = env::attached_deposit();
        let storage_used = env::storage_usage();
        let storage_balance = env::storage_byte_cost() * Balance::from(storage_used);
        (deposit_amount, storage_balance)
    }

    #[payable]
    pub fn withdraw(&mut self, receiver: AccountId, amount: U128) -> Promise {
        assert_one_yocto();
        self.assert_only_owner();

        Promise::new(receiver).transfer(amount.into())
    }
}