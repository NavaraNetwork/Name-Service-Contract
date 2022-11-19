use crate::*;

/// Supported netword domain enum.
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Network {
    Ethereum,
    NEAR,
    Polkadot,
    Solana,
    Terra,
    Cardano,
    Tron,
    Bitcoin
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Network::Ethereum => write!(f, "Ethereum"),
            Network::NEAR => write!(f, "NEAR"),
            Network::Polkadot => write!(f, "Polkadot"),
            Network::Solana => write!(f, "Solana"),
            Network::Terra => write!(f, "Terra"),
            Network::Cardano => write!(f, "Cardano"),
            Network::Tron => write!(f, "Tron"),
            Network::Bitcoin => write!(f, "Bitcoin"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AddressInput {
    pub network: Network,
    pub address: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AddressRemoveInput {
    pub network: Network,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn insert_addresses(&mut self, token_id: TokenId, addresses_input: Vec<AddressInput>) {

        let initial_storage_usage = env::storage_usage();
        // assert if function caller is not the owner
        self.assert_token_owner(&token_id);

        self.assert_token_expires(&token_id);

        // get the token_addresses object by token_id
        let mut token_addresses = if let Some(addresses_by_id) = self.addresses_by_token_id.get(&token_id) {
            addresses_by_id
        } else {
            let token_addresses_hashmap: HashMap<String, String> = HashMap::new();
            token_addresses_hashmap
        };

        // insert into the token_addresses object
        for input in addresses_input.iter() {

            // insert into token_addresses object
            token_addresses.insert(input.network.to_string(), input.address.to_string());
        };

        // override token_addresses object
        self.addresses_by_token_id.insert(&token_id, &token_addresses);

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }

    pub fn get_token_addresses(&self, token_id: TokenId) -> HashMap<String, String> {
        if self.is_token_expires(&token_id) {
            return HashMap::new();
        } else {
            // return the token addresses object
            self.addresses_by_token_id.get(&token_id).unwrap_or_default()
        }
    }

    pub fn get_address(&self, token_id: TokenId, network: Network) -> Option<String> {
        // check expires
        if self.is_token_expires(&token_id) {
            return None;
        }

        // get the token address object by token id
        match self.addresses_by_token_id.get(&token_id) {
            Some(address_by_token_id) => {
                match address_by_token_id.get(&network.to_string()) {
                    Some(address) => {
                        Some(address.to_string())
                    },
                    None => None
                }

            },
            None => {
                None
            }
        }
    }

    #[payable]
    pub fn remove_addresses(&mut self, token_id: TokenId, addresses_input: Vec<AddressRemoveInput>) {
        assert_one_yocto();
        let is_token_expires = self.is_token_expires(&token_id);
        if is_token_expires {
            self.assert_only_owner();
        } else {
            self.assert_token_owner(&token_id);
        }
        let mut token_addresses = self.addresses_by_token_id.get(&token_id).expect("NO_ADDRESSES_TOKEN");
        for input in addresses_input.iter() {
            token_addresses.remove(&input.network.to_string());
        };
        self.addresses_by_token_id.insert(&token_id, &token_addresses);
    }

    pub fn reset_token_addresses(&mut self, token_id: TokenId) {
        assert_one_yocto();
        
        // assert if function caller is not the owner
        self.assert_token_owner(&token_id);
        

        self.addresses_by_token_id.remove(&token_id);
    }
}