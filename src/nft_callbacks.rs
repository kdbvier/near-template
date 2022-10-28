use crate::*;
pub trait NonFungibleTokenReceiver {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool>;
}

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        // println!(
        //     "Info: {:?}, {:?}, {:?}, {:?}, {:?}",
        //     sender_id, previous_owner_id, token_id, nft_contract_id, signer_id
        // );
        // let storage_amount = self.storage_minimum_balance().0;
        // let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
        // let signer_storage_required =
        //     (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount;

        // assert!(
        //     owner_paid_storage > signer_storage_required,
        //     "Insufficient storage paid"
        // );

        self._internal_receive_nft(nft_contract_id, sender_id, token_id);
        PromiseOrValue::Value(false)
    }
}
