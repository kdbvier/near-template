use crate::*;
use near_sdk::json_types::U128;
use near_sdk::PromiseOrValue;
use near_sdk::{serde_json, Promise};
use std::convert::TryInto;

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nft_contract_id: AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ft_token_id: AccountId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: TokenId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: String,
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Callback on receiving tokens by this contract.
    /// transfer reward token with specific msg indicate
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // assert!(self.data().state == RunningState::Running, "{}", ERR600_CONTRACT_PAUSED);

        let sender: AccountId = sender_id.into();
        let amount: u128 = amount.into();
        println!("Sender {}", sender);
        assert!(msg.is_empty() == false, "Empty Message");

        let TokenInfo {
            nft_contract_id,
            ft_token_id,
            token_id,
            method,
        } = near_sdk::serde_json::from_str(&msg).expect("Not valid TokneInfoArgs");

        println!("Info: {:?}, {:?}", nft_contract_id, ft_token_id);
        // if method == "auction" {
        //     self.internal_ft_token_add_bid(nft_contract_id, ft_token_id, token_id, sender, amount.into());
        // } else if method == "buy" {
        //     self.internal_buy(nft_contract_id, token_id, ft_token_id, sender, amount.into());
        // }
        println!("FT Transfer Call");
        PromiseOrValue::Value(U128(0))
    }
}
