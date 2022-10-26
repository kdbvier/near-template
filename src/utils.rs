
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, ext_contract, Gas, Timestamp, PromiseResult};

pub type TimestampSec = u32;

pub const MIN_SEED_DEPOSIT: u128 = 1_000_000_000_000_000_000;
pub const MAX_ACCOUNT_LENGTH: u128 = 64;
/// Amount of gas for fungible token transfers.
pub const GAS_FOR_FT_TRANSFER: u128 = 10_000_000_000_000;
/// Amount of gas for fungile token mint
pub const GAS_FOR_FT_MINT: u128 = 30_000_000_000_000;
pub const GAS_FOR_RESOLVE_FT_MINT: u128 = 10_000_000_000_000;
/// Amount of gas for reward token transfers resolve.
pub const GAS_FOR_RESOLVE_TRANSFER: u128 = 10_000_000_000_000;
/// Amount of gas for seed token transfers resolve.
pub const GAS_FOR_RESOLVE_WITHDRAW_SEED: u128 = 80_000_000_000_000;
pub const MFT_TAG: &str = "@";


/// Assert that 1 yoctoNEAR was attached.
pub fn assert_one_yocto() {
    assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR")
}

pub(crate) fn to_sec(timestamp: Timestamp) -> TimestampSec {
    (timestamp / 10u64.pow(9)) as u32
}