#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod psp22 {
    use ink_storage::traits::SpreadAllocate;
    use ink_lang::codegen::Env;
    use ink_lang::codegen::EmitEvent;
    use openbrush::{
        contracts::psp22::*,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Psp22 {
        #[storage_field]
        psp22: psp22::Data,
        // fields for hater logic
        hated_account: AccountId,
        price: u128,
    }

    impl psp22::Transfer for Psp22 {
        // Let's override method to reject transactions to bad account
        fn _before_token_transfer(
            &mut self,
            _from: Option<&AccountId>,
            to: Option<&AccountId>,
            _amount: &Balance,
        ) -> Result<(), PSP22Error> {
            if to == Some(&self.hated_account) {
                return Err(PSP22Error::Custom(String::from("I hate this account!")))
            }
            Ok(())
        }
    }

    impl psp22::Internal for Psp22 {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                from,
                to,
                value: amount,
            });
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value: amount,
            });
        }
    }

    impl PSP22 for Psp22 {}

    impl Psp22 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, price: u128) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Psp22| {
                instance
                    ._mint_to(instance.env().account_id(), total_supply)
                    .expect("Should mint");
                instance.price = price;
            })
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn sell_token(&mut self, amount:u128) {
            if self.price * amount > self.env().transferred_value() {
                return;
            }
            self._transfer_from_to(self.env().account_id(), self.env().caller(),amount, "transfer_data".as_bytes().to_vec(),);
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, hated: AccountId) {
            self.hated_account = hated;
        }

        #[ink(message)]
        pub fn get_hated_account(&self) -> AccountId {
            self.hated_account.clone()
        }

        #[ink(message)]
        pub fn get_total_supply(&self) -> Balance {
            self.total_supply()
        }
    }
}
