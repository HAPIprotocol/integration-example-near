use crate::core_token::{FungibleTokenCore, FungibleTokenInternal};
use crate::resolver::FungibleTokenResolver;
use crate::*;

use near_sdk::{
    assert_one_yocto,
    env::{current_account_id, predecessor_account_id},
    ext_contract, near_bindgen, AccountId, Promise, PromiseOrValue,
};

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[payable]
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> Promise {
        assert_one_yocto();
        let sender: AccountId = predecessor_account_id();
        ext_aml::ext(self.aml.get_account())
            .with_static_gas(AML_CHECK_GAS)
            .get_address(sender.clone())
            .then(
                ext_self::ext(current_account_id())
                    .with_static_gas(CALLBACK_AML_GAS)
                    .cb_ft_transfer(sender, receiver_id, amount, memo),
            )
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_one_yocto();

        let sender: AccountId = predecessor_account_id();
        PromiseOrValue::Promise(
            ext_aml::ext(self.aml.get_account())
                .with_static_gas(AML_CHECK_GAS)
                .get_address(sender.clone())
                .then(
                    ext_self::ext(current_account_id())
                        .with_static_gas(CALLBACK_AML_GAS)
                        .cb_ft_transfer_call(sender, receiver_id, amount, memo, msg),
                ),
        )
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}

#[ext_contract(ext_self)]
pub trait ExtContract {
    /// Callback after ft_transfer.
    fn cb_ft_transfer(
        &mut self,
        sender_id: AccountId,
        #[callback] category_risk: CategoryRisk,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    );

    fn cb_ft_transfer_call(
        &mut self,
        sender_id: AccountId,
        #[callback] category_risk: CategoryRisk,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[near_bindgen]
impl ExtContract for Contract {
    #[private]
    fn cb_ft_transfer(
        &mut self,
        sender_id: AccountId,
        #[callback] category_risk: CategoryRisk,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) {
        self.aml.assert_risk(category_risk);
        self.token.ft_transfer(sender_id, receiver_id, amount, memo)
    }

    #[private]
    fn cb_ft_transfer_call(
        &mut self,
        sender_id: AccountId,
        #[callback] category_risk: CategoryRisk,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.aml.assert_risk(category_risk);
        self.token
            .ft_transfer_call(sender_id, receiver_id, amount, memo, msg)
    }
}

#[near_bindgen]
impl FungibleTokenResolver for Contract {
    #[private]
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        let (used_amount, burned_amount) =
            self.token
                .internal_ft_resolve_transfer(&sender_id, receiver_id, amount);
        if burned_amount > 0 {
            self.on_tokens_burned(sender_id, burned_amount);
        }
        used_amount.into()
    }
}

/// Ensures that when fungible token storage grows by collections adding entries,
/// the storage is be paid by the caller. This ensures that storage cannot grow to a point
/// that the FT contract runs out of Ⓝ.
/// Takes name of the Contract struct, the inner field for the token and optional method name to
/// call when the account was closed.
#[macro_export]
macro_rules! impl_fungible_token_storage {
    ($contract: ident, $token: ident $(, $on_account_closed_fn:ident)?) => {
        use $crate::storage_management::{
            StorageManagement, StorageBalance, StorageBalanceBounds
        };

        #[near_bindgen]
        impl StorageManagement for $contract {
            #[payable]
            fn storage_deposit(
                &mut self,
                account_id: Option<AccountId>,
                registration_only: Option<bool>,
            ) -> StorageBalance {
                self.$token.storage_deposit(account_id, registration_only)
            }

            #[payable]
            fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
                self.$token.storage_withdraw(amount)
            }

            #[payable]
            fn storage_unregister(&mut self, force: Option<bool>) -> bool {
                #[allow(unused_variables)]
                if let Some((account_id, balance)) = self.$token.internal_storage_unregister(force) {
                    $(self.$on_account_closed_fn(account_id, balance);)?
                    true
                } else {
                    false
                }
            }

            fn storage_balance_bounds(&self) -> StorageBalanceBounds {
                self.$token.storage_balance_bounds()
            }

            fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
                self.$token.storage_balance_of(account_id)
            }
        }
    };
}
