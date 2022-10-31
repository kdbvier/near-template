use external::{ext_fungible_token, ext_non_fungible_token};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, ext_contract, near_bindgen, serde_json::json, AccountId, Balance,
    BorshStorageKey, CryptoHash, Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
    Timestamp,
};

mod external;
mod nft_callbacks;
pub type TokenId = String;

const GAS_FOR_FT_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(20_000_000_000_000);
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingInfo {
    pub address: AccountId,
    pub token_ids: Vec<String>,
    pub claimed_amount: u128,
    pub unclaimed_amount: u128,
    pub claimed_timestamp: u64,
    pub create_unstake_timestamp: u64,
    pub last_timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ConfigInfo {
    pub nft_address: AccountId,
    pub ft_address: AccountId,
    pub daily_reward: u128,
    pub interval: u64,
    pub lock_time: u64,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingInfoJson {
    account_id: AccountId,
    staking_info: StakingInfo,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub staking_per_owner: UnorderedMap<AccountId, StakingInfo>,

    pub storage_deposits: LookupMap<AccountId, Balance>,
    pub nft_address: AccountId,
    pub ft_address: AccountId,
    pub daily_reward: u128,
    pub interval: u64,
    pub lock_time: u64,
    pub enabled: bool,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    TokensPerOwner,
    TokensById,
    ConfigData,
    StorageDeposits,
    ByOwnerIdInner { account_id_hash: CryptoHash },
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        nft_address: AccountId,
        ft_address: AccountId,
        daily_reward: u128,
        interval: u64,
        lock_time: u64,
    ) -> Self {
        let this = Self {
            staking_per_owner: UnorderedMap::new(StorageKey::TokensPerOwner),
            owner_id,
            storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
            nft_address,
            ft_address,
            daily_reward,
            interval,
            lock_time,
            enabled: false,
        };

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
        self.enabled = enabled;
    }

    #[payable]
    pub fn update_config(&mut self, config: ConfigInfo) {
        assert_one_yocto();
        self.interval = config.interval;
        self.daily_reward = config.daily_reward;
        self.lock_time = config.lock_time;
        self.ft_address = config.ft_address;
        self.nft_address = config.nft_address;
        self.enabled = config.enabled;
    }

    fn update_unclaimed_amount(&mut self, owner_id: AccountId) {
        let mut user_stake_info = self
            .staking_per_owner
            .get(&owner_id)
            .expect("Marble: You don't have any staking nft");
        if user_stake_info.create_unstake_timestamp == 0u64 {
            user_stake_info.unclaimed_amount += ((to_sec(env::block_timestamp()) as u128
                / self.interval as u128
                - user_stake_info.last_timestamp as u128 / self.interval as u128)
                * (user_stake_info.token_ids.len() as u128))
                * self.daily_reward;
            user_stake_info.last_timestamp = to_sec(env::block_timestamp());
            self.staking_per_owner.insert(&owner_id, &user_stake_info);
        }
    }

    #[payable]
    fn _internal_receive_nft(
        &mut self,
        _nft_contract_id: AccountId,
        _previous_owner_id: AccountId,
        _token_id: String,
    ) {
        assert_eq!(_nft_contract_id, self.nft_address, "Not Allowed NFT");
        let mut record = StakingInfo {
            address: _previous_owner_id.clone(),
            token_ids: vec![_token_id.clone()],
            claimed_amount: 0u128,
            unclaimed_amount: 0u128,
            claimed_timestamp: to_sec(env::block_timestamp()),
            create_unstake_timestamp: 0u64,
            last_timestamp: to_sec(env::block_timestamp()),
        };
        if self.staking_per_owner.get(&_previous_owner_id).is_some() {
            self.update_unclaimed_amount(_previous_owner_id.clone());
            record = self
                .staking_per_owner
                .get(&_previous_owner_id)
                .expect("Marble: Token doesn't exist");
            let mut list = record.token_ids.clone();
            list.push(_token_id.clone());
            record.token_ids = list;
        }
        self.staking_per_owner.insert(&_previous_owner_id, &record);

        env::log_str(
            &json!({
                "type": "stake_nft",
                "params": {
                    "owner_id": _previous_owner_id,
                    "nft_contract_id": _nft_contract_id,
                    "token_id": _token_id
                }
            })
            .to_string(),
        )
    }

    #[payable]
    pub fn claim_rewards(&mut self) {
        assert_one_yocto();
        let account = env::predecessor_account_id();
        self.update_unclaimed_amount(account.clone());
        let mut staking_info = self
            .staking_per_owner
            .get(&account)
            .expect("Marble: You don't have any staked Nfts.");

        assert_ne!(
            staking_info.unclaimed_amount, 0u128,
            "Marble: You don't have any reward"
        );
        ext_fungible_token::ft_transfer(
            account.clone(),
            staking_info.unclaimed_amount.into(),
            None,
            self.ft_address.clone(),
            1,
            GAS_FOR_FT_TRANSFER,
        )
        .then(ext_self::callback_post_withdraw_deposit(
            self.ft_address.clone(),
            staking_info.unclaimed_amount,
            env::current_account_id(),
            0,
            GAS_FOR_FT_TRANSFER,
        ));
        staking_info.claimed_amount += staking_info.unclaimed_amount;
        staking_info.unclaimed_amount = 0u128;
        staking_info.claimed_timestamp = to_sec(env::block_timestamp());
        self.staking_per_owner.insert(&account, &staking_info);
    }

    #[payable]
    pub fn create_unstake(&mut self) {
        assert_one_yocto();
        let account = env::predecessor_account_id();
        self.update_unclaimed_amount(account.clone());
        let mut staking_info = self
            .staking_per_owner
            .get(&account)
            .expect("Marble: No nfts");
        staking_info.create_unstake_timestamp = to_sec(env::block_timestamp());

        self.staking_per_owner.insert(&account, &staking_info);
        env::log_str(
            &json!({
                "type": "create_unstake",
                "params": {
                    "owner_id": account,
                }
            })
            .to_string(),
        )
    }

    #[payable]
    pub fn fetch_unstake(&mut self) {
        let account = env::predecessor_account_id();
        self.update_unclaimed_amount(account.clone());
        let staking_info = self
            .staking_per_owner
            .get(&account)
            .expect("Marble: no nft");
        assert_ne!(
            staking_info.create_unstake_timestamp, 0u64,
            "Marble: Create unstake first"
        );
        assert!(
            to_sec(env::block_timestamp()) > self.lock_time + staking_info.create_unstake_timestamp,
            "Marble: Still in lock"
        );
        if staking_info.unclaimed_amount > 0u128 {
            ext_fungible_token::ft_transfer(
                account.clone(),
                staking_info.unclaimed_amount.into(),
                None,
                self.ft_address.clone(),
                1,
                GAS_FOR_FT_TRANSFER,
            )
            .then(ext_self::callback_post_withdraw_deposit(
                self.ft_address.clone(),
                staking_info.unclaimed_amount,
                env::current_account_id(),
                0,
                GAS_FOR_FT_TRANSFER,
            ));
        }

        for token_id in staking_info.token_ids.clone() {
            ext_non_fungible_token::nft_transfer(
                staking_info.address.clone(),
                token_id.clone(),
                None,
                None,
                self.nft_address.clone(),
                1,
                GAS_FOR_NFT_TRANSFER,
            );
            // .then(ext_self::nft_unstaking_callback(
            //     account.clone(),
            //     token_id.clone(),
            //     env::current_account_id(),
            //     1,
            //     GAS_FOR_NFT_TRANSFER,
            // ));
            env::log_str(
                &json!({
                    "type": "unstake_nft",
                    "params": {
                        "owner_id": staking_info.address,
                        "token_id": token_id,
                    }
                })
                .to_string(),
            );
        }
        self.staking_per_owner.remove(&account);
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_config(&self) -> ConfigInfo {
        ConfigInfo {
            nft_address: self.nft_address.clone(),
            ft_address: self.ft_address.clone(),
            daily_reward: self.daily_reward,
            interval: self.interval,
            lock_time: self.lock_time,
            enabled: self.enabled,
        }
    }

    pub fn get_enable_status(&self) -> bool {
        self.enabled.clone()
    }

    pub fn get_total_amount(&self) -> Promise {
        ext_fungible_token::ft_balance_of(
            env::current_account_id(),
            self.ft_address.clone(),
            0,
            Gas(10_000_000_000_000),
        )
        .then(ext_self::ft_balance_of_callback(
            env::current_account_id(),
            0,
            Gas(10_000_000_000_000),
        ))
    }

    pub fn get_stake_info(&self, owner: AccountId) -> StakingInfo {
        self.staking_per_owner
            .get(&owner)
            .expect("Marble: You don't have any staked nfts.")
    }

    pub fn get_all_stake_info(
        &self,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<StakingInfoJson> {
        let start_index: u128 = from_index.map(From::from).unwrap_or_default();
        assert!(
            (self.staking_per_owner.len() as u128) > start_index,
            "Out of bounds, please use a smaller from_index."
        );
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");
        self.staking_per_owner
            .iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|(account, stake_info)| StakingInfoJson {
                account_id: account,
                staking_info: stake_info,
            })
            .collect()
    }

    #[private]
    pub fn ft_balance_of_callback(&mut self) -> String {
        assert_eq!(env::promise_results_count(), 1, "This is a callback method");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "Marble: ft balance error".to_string(),
            PromiseResult::Successful(result) => {
                let balance = near_sdk::serde_json::from_slice::<U128>(&result).unwrap();
                balance.0.to_string()
            }
        }
    }

    #[private]
    pub fn callback_post_withdraw_deposit(&mut self, owner_id: AccountId, amount: u128) -> U128 {
        env::log_str(
            &json!({
                "type": "claim_reward",
                "params": {
                    "owner_id": owner_id,
                    "amount": amount,
                }
            })
            .to_string(),
        );
        U128(0)
    }

    // #[private]
    // pub fn nft_unstaking_callback(&mut self, owner_id: AccountId, token_id: String) -> U128 {
    //     env::log_str(
    //         &json!({
    //             "type": "unstake_nft",
    //             "params": {
    //                 "owner_id": owner_id,
    //                 "token_id": token_id,
    //             }
    //         })
    //         .to_string(),
    //     );
    //     U128(0)
    // }

    // pub fn get_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
    //     self.staking_per_owner
    //         .get(&account_id)
    //         .map_or(0, |by_owner_id| by_owner_id.token_ids.len())
    //         .into()
    // }

    // Storage
    // #[payable]
    // pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
    //     let storage_account_id = account_id
    //         .map(|a| a.into())
    //         .unwrap_or_else(env::predecessor_account_id);
    //     let deposit = env::attached_deposit();
    //     assert!(
    //         deposit >= STORAGE_ADD_STAKING_DATA,
    //         "Requires minimum deposit of {}",
    //         STORAGE_ADD_STAKING_DATA
    //     );

    //     let mut balance: u128 = self.storage_deposits.get(&storage_account_id).unwrap_or(0);
    //     balance += deposit;
    //     self.storage_deposits.insert(&storage_account_id, &balance);
    // }

    // #[payable]
    // pub fn storage_withdraw(&mut self) {
    //     assert_one_yocto();
    //     let owner_id = env::predecessor_account_id();
    //     let mut amount = self.storage_deposits.remove(&owner_id).unwrap_or(0);
    //     let market_data_owner = self.staking_per_owner.get(&owner_id);
    //     let len = market_data_owner.map(|s| s.len()).unwrap_or_default();
    //     let diff = u128::from(len) * STORAGE_ADD_STAKING_DATA;
    //     amount -= diff;
    //     if amount > 0 {
    //         Promise::new(owner_id.clone()).transfer(amount);
    //     }
    //     if diff > 0 {
    //         self.storage_deposits.insert(&owner_id, &diff);
    //     }
    // }

    // pub fn storage_minimum_balance(&self) -> u128 {
    //     u128(STORAGE_ADD_STAKING_DATA)
    // }

    // pub fn storage_balance_of(&self, account_id: AccountId) -> u128 {
    //     self.storage_deposits.get(&account_id).unwrap_or(0).into()
    // }
}

pub fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}
pub fn to_sec(timestamp: Timestamp) -> u64 {
    (timestamp / 10u64.pow(9)) as u64
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn ft_balance_of_callback(&mut self) -> String;
    fn callback_post_withdraw_deposit(&mut self, owner_id: AccountId, amount: u128) -> U128;
    fn nft_unstaking_callback(&mut self, owner_id: AccountId, token_id: String) -> U128;
}

// use the attribute below for unit tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    // fn get_context(predecessor: AccountId) -> VMContextBuilder {
    //     let mut builder = VMContextBuilder::new();
    //     builder.predecessor_account_id(predecessor);
    //     builder
    // }
    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new(
            accounts(0),
            accounts(1),
            accounts(2),
            100,
            1000000,
            1000000000,
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
    #[test]
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

    #[test]
    fn internal_receive_nft_test() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            // .attached_deposit(1)
            .build());
        let config = contract.get_config();
        println!(
            "Config status: {:?}, {:?}, {:?},{:?},{:?}",
            config.nft_address,
            config.ft_address,
            config.daily_reward,
            config.interval,
            config.lock_time
        );
        // let mut supply = contract.get_supply_by_owner_id(accounts(3));
        // println!("first supply: {:?}", supply);
        // contract._internal_receive_nft(accounts(1), accounts(3), "1:1".to_string());
        // supply = contract.get_supply_by_owner_id(accounts(3));
        // println!("second supply: {:?}", supply);
        // let token_info: StakingInfo = contract.get_token_by_id("1:1".to_string());
        // println!(
        //     "token-info: {:?}, {:?}, {:?}, {:?}, {:?}",
        //     token_info.address,
        //     token_info.claimed_amount,
        //     token_info.token_id,
        //     token_info.unclaimed_amount,
        //     token_info.last_timestamp
        // );
    }
}
