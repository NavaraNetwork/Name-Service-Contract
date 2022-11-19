use crate::*;

#[near_bindgen]
impl Contract {
    pub fn nft_price_per_year(&self) -> Balance {
        self.price_per_year
    }

    pub fn set_token_price_per_year(&mut self, price_per_year: Balance) {
        self.assert_only_owner();
        assert!(price_per_year > DEFAULT_PRICE_PER_YEAR, "price is at least: {}", DEFAULT_PRICE_PER_YEAR);
        self.price_per_year = price_per_year
    }
}