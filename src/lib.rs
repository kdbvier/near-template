use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, ValidAccountId, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance, CryptoHash,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};
use std::collections::HashMap;

mod external;
mod nft_callbacks;
mod token_receiver;
mod utils;

pub const STORAGE_ADD_STAKING_DATA: u128 = 8590000000000000000000;

pub type TokenId = String;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub address: AccountId,
    pub token_id: String,
    pub claimed_amount: U128,
    pub unclaimed_amount: U128,
    pub claimed_timestamp: u64,
    pub create_unstake_timestamp: u64,
    pub last_timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ConfigData {
    pub nft_address: String, // required, nft token address
    pub ft_address: String,  // required, ft token address
    pub daily_reward: U128,  // required, the amount of the daily reward
    pub interval: u64,       // interval time
    pub lock_time: u64,      // lock time
    pub enabled: bool,       // staking enable
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<String>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<String, Token>,

    //keeps track of the metadata for the contract
    pub config: ConfigData,

    pub storage_deposits: LookupMap<AccountId, Balance>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokensById,
    ConfigData,
    StorageDeposits,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId, configdata: ConfigData) -> Self {
        //create a variable of type Self with all the fields initialized.
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            //set the owner_id field equal to the passed in owner_id.
            owner_id,
            config: configdata,
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits.try_to_vec().unwrap()),
        };

        //return the Contract object
        this
    }

    #[payable]
    pub fn update_owner(&mut self, owner_id: AccountId) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Marble: Owner only"
        );
        self.owner_id = owner_id;
    }

    #[payable]
    pub fn update_enable(&mut self, enabled: bool) {
        assert_one_yocto();
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Marble: Owner only"
        );
        self.config.enabled = enabled;
    }

    #[payable]
    fn _internal_receive_nft(
        &mut self,
        _nft_contract_id: AccountId,
        _previous_owner_id: AccountId,
        _token_id: String,
    ) {
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_enable_status(&self) -> bool {
        self.config.enabled.clone()
    }
    pub fn get_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
        self.tokens_per_owner
            .get(&account_id)
            .map_or(0, |by_owner_id| by_owner_id.len())
            .into()
    }
    // Storage
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        let storage_account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(env::predecessor_account_id);
        let deposit = env::attached_deposit();
        assert!(
            deposit >= STORAGE_ADD_STAKING_DATA,
            "Requires minimum deposit of {}",
            STORAGE_ADD_STAKING_DATA
        );

        let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
        balance += deposit;
        self.storage_deposits.insert(&storage_account_id, &balance);
    }

    #[payable]
    pub fn storage_withdraw(&mut self) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);
        let market_data_owner = self.tokens_per_owner.get(&owner_id);
        let len = market_data_owner.map(|s| s.len()).unwrap_or_default();
        let diff = u128::from(len) * STORAGE_ADD_STAKING_DATA;
        amount -= diff;
        if amount > 0 {
            Promise::new(owner_id.clone()).transfer(amount);
        }
        if diff > 0 {
            self.storage_deposits.insert(&owner_id, &diff);
        }
    }

    pub fn storage_minimum_balance(&self) -> U128 {
        U128(STORAGE_ADD_STAKING_DATA)
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        self.storage_deposits.get(&account_id).unwrap_or(0).into()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
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
        let contract = Contract::new(
            accounts(0),
            ConfigData {
                nft_address: accounts(1).to_string(),
                ft_address: accounts(2).to_string(),
                daily_reward: U128::from(100),
                interval: 1000000,
                lock_time: 1000000000,
                enabled: false,
            },
        );
        (context, contract)
    }

    #[test]
    fn test_new() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        assert_eq!(contract.get_owner(), accounts(0));
        contract.update_owner(accounts(1));
        assert_eq!(contract.get_owner(), accounts(1));
    }
    fn test_enable() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());
        assert_eq!(contract.get_enable_status(), false);
        contract.update_enable(true);
        assert_eq!(contract.get_enable_status(), true);
    }
}
