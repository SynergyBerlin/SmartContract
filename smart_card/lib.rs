#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod smart_card {
    use ink_env::AccountId;
    use ink_storage::{
        traits::{SpreadAllocate, SpreadLayout},
        Mapping,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct SmartCard {
        owner: AccountId,
        spending_limit: Balance,
        spent_amount: Balance,
        delegates: Mapping<AccountId, ()>, // simple set of delegates
    }

    impl SmartWallet {
        #[ink(constructor)]
        pub fn new(owner: AccountId, spending_limit: Balance) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.owner = owner;
                contract.spending_limit = spending_limit;
                contract.spent_amount = 0;
            })
        }

        /// Owner adds a delegate
        #[ink(message)]
        pub fn add_delegate(&mut self, delegate: AccountId) {
            assert_eq!(self.env().caller(), self.owner, "Not owner");
            self.delegates.insert(delegate, &());
        }

        /// Owner removes a delegate
        #[ink(message)]
        pub fn remove_delegate(&mut self, delegate: AccountId) {
            assert_eq!(self.env().caller(), self.owner, "Not owner");
            self.delegates.remove(delegate);
        }

        /// Checks if an address is delegate
        fn is_delegate(&self, addr: &AccountId) -> bool {
            self.delegates.contains(addr)
        }

        /// Update spending limit (owner only)
        #[ink(message)]
        pub fn update_spending_limit(&mut self, new_limit: Balance) {
            assert_eq!(self.env().caller(), self.owner, "Not owner");
            self.spending_limit = new_limit;
        }

        /// Transfer funds to a target if within spending limit by owner or delegate.
        #[ink(message, payable)]
        pub fn transfer(&mut self, to: AccountId, amount: Balance) -> bool {
            let caller = self.env().caller();

            // Only owner or delegates allowed
            assert!(
                caller == self.owner || self.is_delegate(&caller),
                "Not authorized"
            );

            assert!(self.env().balance() >= amount, "Insufficient funds");

            // Check spending limit
            assert!(
                self.spent_amount + amount <= self.spending_limit,
                "Spending limit exceeded"
            );

            // Update spent amount
            self.spent_amount += amount;

            // Transfer balance
            self.env().transfer(to, amount).expect("Transfer failed");
            true
        }

        /// Get current balance
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }

        /// Get spending info
        #[ink(message)]
        pub fn get_spending_info(&self) -> (Balance, Balance, Balance) {
            (self.spending_limit, self.spent_amount, self.spending_limit - self.spent_amount)
        }

        /// Check if address is delegate
        #[ink(message)]
        pub fn is_delegate_address(&self, addr: AccountId) -> bool {
            self.is_delegate(&addr)
        }

        /// Get owner
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }
} 