use crate::*;
trait NonFungibleOnTransfer {
    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise;

    // fn nft_resolve_transfer(
    //     &mut self,
    //     //we introduce an authorized ID for logging the transfer event
    //     authorized_id: Option<String>,
    //     owner_id: AccountId,
    //     receiver_id: AccountId,
    //     token_id: TokenId,
    //     //we introduce the approval map so we can keep track of what the approvals were before the transfer
    //     approved_account_ids: HashMap<AccountId, u64>,
    //     //we introduce a memo for logging the transfer event
    //     memo: Option<String>,
    // ) -> bool;
}

#[near_bindgen]
impl NonFungibleOnTransfer for Contract {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> Promise {
        // enforce cross contract call and owner_id is signer

        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        println!(
            "Info: {:?}, {:?}, {:?}, {:?}, {:?}",
            sender_id, previous_owner_id, token_id, nft_contract_id, signer_id
        );
        let storage_amount = self.storage_minimum_balance().0;
        let owner_paid_storage = self.storage_deposits.get(&signer_id).unwrap_or(0);
        let signer_storage_required =
            (self.get_supply_by_owner_id(signer_id).0 + 1) as u128 * storage_amount;

        assert!(
            owner_paid_storage > signer_storage_required,
            "Insufficient storage paid"
        );
        // if owner_paid_storage < signer_storage_required {
        //     let notif = format!(
        //         "Insufficient storage paid: {}, for {} sales at {} rate of per sale",
        //         owner_paid_storage,
        //         signer_storage_required / storage_amount,
        //         storage_amount
        //     );
        //     env::log_str(&notif);
        //     return PromiseOrValue::Value(false);
        // };
        Promise::new("bob_near".parse().unwrap())
            .create_account()
            .transfer(0)
            .add_full_access_key(env::signer_account_pk())
    }
}
