use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};


use crate::internal::*;
use crate::utils::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::address::*;
pub use crate::ttl::*;
pub use crate::price::*;
pub use crate::storage_manage::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty; 
mod events;
mod address;
mod utils;
mod ttl;
mod price;
mod storage_manage;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

const ONE_YEAR_NANOSECOND: u64 = 31_536_000_000_000_000u64;

const ONE_NEAR_ES_YOCTO: Balance = 1_000_000_000_000_000_000_000_000;

const DEFAULT_PRICE_PER_YEAR: Balance = ONE_NEAR_ES_YOCTO / 5;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    //keeps track of token address
    pub addresses_by_token_id : UnorderedMap<TokenId, HashMap<String, String>>,

    // nft price per year
    pub price_per_year: Balance

}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshDeserialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    AddressesByTokenId
}

#[near_bindgen]
impl Contract {

    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Dnet NFT".to_string(),
                symbol: "DNFT".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            addresses_by_token_id: UnorderedMap::new(StorageKey::AddressesByTokenId.try_to_vec().unwrap()),
            price_per_year: DEFAULT_PRICE_PER_YEAR
        };

        //return the Contract object
        this
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this: Self = env::state_read().expect("Cannot deserialize");

        assert_eq!(
            env::predecessor_account_id(),
            this.owner_id,
            "Only owner"
        );

        Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            addresses_by_token_id: UnorderedMap::new(StorageKey::AddressesByTokenId.try_to_vec().unwrap()),
            price_per_year: this.price_per_year
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env};
    use near_sdk::json_types::{ValidAccountId, U128, U64};
    const STORAGE_FOR_MINT: Balance = 11280000000000000000000;

    

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new_default_meta(accounts(0));
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(1),
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Triple Triad".to_string(),
                symbol: "TRIAD".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            }
        );
        testing_env!(context.is_view(true).build());
    }

    #[test]
    fn test_mint() {
        let (mut context, mut contract) = setup_contract();

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1), 1000);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(ONE_NEAR_ES_YOCTO / 5)
            .build()
        );

        let token_id = "manhnv".to_string();
        contract.nft_mint(
            token_id.clone(), 
            TokenMetadata {
                title: Some("manhnv".to_string()), 
                description: Some("bitcoin domain provided by decentrailnet".to_string()), 
                extra: Some("0x33ed1b1B29e807fCf15EC731b0c0DE18d306be1a".to_string()), 
                media: Some("https://vcdn-sohoa.vnecdn.net/2022/03/08/bored-ape-nft-accidental-0-728-5490-8163-1646708401.jpg".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                reference: None,
                reference_hash: None,
            },
            accounts(1),
            Some(royalty),
        );

        let token_from_nft_token = contract.nft_token(token_id);
        assert_eq!(
            token_from_nft_token.unwrap().owner_id.to_string(),
            accounts(1).to_string()
        );
    }

    #[test]
    fn test_add_address() {
        let (mut context, mut contract) = setup_contract();

        let mut royalty: HashMap<AccountId, u32> = HashMap::new();
        royalty.insert(accounts(1), 1000);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(ONE_NEAR_ES_YOCTO / 5)
            .build()
        );

        let token_id = "manhnv".to_string();
        contract.nft_mint(
            token_id.clone(), 
            TokenMetadata {
                title: Some("manhnv".to_string()), 
                description: Some("bitcoin domain provided by decentrailnet".to_string()), 
                extra: Some("0x33ed1b1B29e807fCf15EC731b0c0DE18d306be1a".to_string()), 
                media: Some("https://vcdn-sohoa.vnecdn.net/2022/03/08/bored-ape-nft-accidental-0-728-5490-8163-1646708401.jpg".to_string()),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                reference: None,
                reference_hash: None,
            },
            accounts(1),
            Some(royalty),
        );

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(true)
            .build()
        );

        let token_from_nft_token = contract.nft_token(token_id.clone());

        let expires_time = contract.nft_token_expires(token_id.clone());

        let ethereum_address = "0x33ed1b1B29e807fCf15EC731b0c0DE18d306be1a".to_string();
        let remind_word = "wallet 1".to_string();

        let addresses = vec![
            AddressInput {
                network: Network::Ethereum,
                address: ethereum_address.clone()
            },
            AddressInput {
                network: Network::NEAR,
                address: ethereum_address.clone()
            }
        ];

        let addresses_remove = vec![
            AddressRemoveInput {
                network: Network::Ethereum,
            },
        ];

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(false)
            .build()
        );

        contract.insert_addresses(token_id.clone(), addresses);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(true)
            .build()
        );

        let address1 = contract.get_address(token_id.clone(), Network::Ethereum);
        assert!(address1.is_some(), "NOT_FOUND");
        let address2 = contract.get_address(token_id.clone(), Network::NEAR);
        assert!(address2.is_some(), "NOT_FOUND");
        let address3 = contract.get_address(token_id.clone(), Network::Polkadot);
        assert!(address3.is_none(), "NOT_FOUND");

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(false)
            .build()
        );

        contract.remove_addresses(token_id.clone(), addresses_remove);

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(true)
            .build()
        );

        let address2 = contract.get_address(token_id.clone(), Network::Ethereum);
        assert!(address2.is_none(), "NOT_FOUND");

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(false)
            .build()
        );

        contract.reset_token_addresses(token_id.clone());

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(true)
            .build()
        );

        let address3 = contract.get_address(token_id.clone(), Network::NEAR);
        assert!(address3.is_none(), "NOT_FOUND");

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .is_view(false)
            .attached_deposit(DEFAULT_PRICE_PER_YEAR * 10 + DEFAULT_PRICE_PER_YEAR/77)
            .build()
        );

        let (expires, change) = contract.extend_token(token_id.clone());

        let new_expires_date = contract.nft_token_expires(token_id.clone());
        assert_eq!(
            new_expires_date,
            expires,
            "NOT_EQUAL {} - {}",
            new_expires_date,
            expires
        );

        assert_eq!(
            DEFAULT_PRICE_PER_YEAR/77,
            change,
            "NOT_EQUAL 2 {} - {}",
            new_expires_date,
            expires
        );

        assert_eq!(
            new_expires_date / ONE_YEAR_NANOSECOND,
            11,
            "NOT_EQUAL 3 {} - {}",
            new_expires_date,
            11
        );
    }

    #[test]
    fn test_price() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .is_view(true)
            .attached_deposit(DEFAULT_PRICE_PER_YEAR * 10 + DEFAULT_PRICE_PER_YEAR/77)
            .build()
        );

        let price_per_year = contract.nft_price_per_year();

        assert_eq!(
            price_per_year,
            DEFAULT_PRICE_PER_YEAR,
            "Price per year mismatch {} - {}",
            price_per_year,
            DEFAULT_PRICE_PER_YEAR,
        );

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .is_view(false)
            // .attached_deposit(DEFAULT_PRICE_PER_YEAR * 10 + DEFAULT_PRICE_PER_YEAR/77)
            .build()
        );

        let new_price = DEFAULT_PRICE_PER_YEAR * 2;

        let new_price_per_year = contract.set_token_price_per_year(new_price);

        let price_per_year = contract.nft_price_per_year();

        assert_eq!(
            price_per_year,
            new_price,
            "NEW Price per year mismatch {} - {}",
            price_per_year,
            new_price,
        );
    }

    #[test]
    fn test_withdraw() {
        let (mut context, mut contract) = setup_contract();

        

        let balance_before = env::account_balance();

        let withdraw_amount = 5* ONE_NEAR_ES_YOCTO;

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .is_view(false)
            .attached_deposit(1)
            .build()
        );

        let price_per_year = contract.withdraw(accounts(0), U128::from(withdraw_amount));

        let balance_after = env::account_balance();

        assert_eq!(
            balance_after - balance_before,
            withdraw_amount,
            "WITHDRAW_BALANCE_MISMATCH: {} != {}, before: {}, after: {}",
            balance_after - balance_before,
            withdraw_amount,
            balance_before,
            balance_after
        );

        // let price_per_year = contract.withdraw(accounts(0), 100 * ONE_NEAR_ES_YOCTO);
    }
}
