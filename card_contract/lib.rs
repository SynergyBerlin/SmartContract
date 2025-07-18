#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// ## Card Contract (Account Abstraction)
#[ink::contract(env = ink::env::DefaultEnvironment)]
pub mod card_contract {
    use ink::env::call::{build_call, ExecutionInput, Selector};
    use ink::env::DefaultEnvironment;
    use parity_scale_codec::{Encode, Decode};
    use ink::primitives::H160;
    use ink::primitives::U256;
    use scale_info::TypeInfo;

    #[ink(storage)]
    pub struct CardContract {
        owner: H160,
        delegate: H160,
        // Option 1: Cumulative cap
        total_spent: U256,
        spending_cap: U256,
        // Option 2: Per-tx limit
        per_tx_limit: U256,
        // Option 3: Quota with reset
        quota: U256,
        quota_reset_interval: u64, // in seconds
        quota_spent: U256,
        last_reset: u64, // timestamp (ms)
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    pub enum Error {
        NotDelegate,
        NotOwner,
        TransferFailed,
        ExceedsSpendingCap,
        ExceedsPerTxLimit,
        ExceedsQuota,
    }

    impl CardContract {
        #[ink(constructor)]
        pub fn new(
            delegate: H160,
            spending_cap: U256,
            per_tx_limit: U256,
            quota: U256,
            quota_reset_interval: u64,
        ) -> Self {
            let caller = Self::env().caller();
            let actual_delegate = if delegate == H160::zero() { caller } else { delegate };
            let now = Self::env().block_timestamp();
            Self {
                owner: caller,
                delegate: actual_delegate,
                total_spent: U256::from(0),
                spending_cap,
                per_tx_limit,
                quota,
                quota_reset_interval,
                quota_spent: U256::from(0),
                last_reset: now,
            }
        }

        #[ink(message)]
        pub fn pay(
            &mut self,
            intermediary_contract: H160,
            amount: U256,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.delegate && caller != self.owner {
                return Err(Error::NotDelegate);
            }

            // Option 3: Reset quota if needed
            let now = Self::env().block_timestamp();
            if now >= self.last_reset + self.quota_reset_interval * 1000 {
                self.quota_spent = U256::from(0);
                self.last_reset = now;
            }

            // Option 1: Cumulative cap
            if self.total_spent + amount > self.spending_cap {
                return Err(Error::ExceedsSpendingCap);
            }

            // Option 2: Per-tx limit
            if amount > self.per_tx_limit {
                return Err(Error::ExceedsPerTxLimit);
            }

            // Option 3: Quota
            if self.quota_spent + amount > self.quota {
                return Err(Error::ExceedsQuota);
            }

            // Proceed with payment
            let selector_bytes = [0x63, 0x6F, 0x69, 0x6E];
            build_call::<DefaultEnvironment>()
                .call(intermediary_contract)
                .transferred_value(amount)
                .exec_input(ExecutionInput::new(Selector::new(selector_bytes)))
                .returns::<()>()
                .invoke();

            self.total_spent += amount;
            self.quota_spent += amount;
            Ok(())
        }

        #[ink(message)]
        pub fn set_delegate(&mut self, new_delegate: H160) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            self.delegate = new_delegate;
            Ok(())
        }

        #[ink(message)]
        pub fn get_owner(&self) -> H160 {
            self.owner
        }


        #[ink(message)]
        pub fn get_delegate(&self) -> H160 {
            self.delegate
        }

        #[ink(message)]
        pub fn get_total_spent(&self) -> U256 {
            self.total_spent
        }

        #[ink(message)]
        pub fn get_spending_cap(&self) -> U256 {
            self.spending_cap
        }

        #[ink(message)]
        pub fn get_per_tx_limit(&self) -> U256 {
            self.per_tx_limit
        }

        #[ink(message)]
        pub fn get_quota(&self) -> U256 {
            self.quota
        }

        #[ink(message)]
        pub fn get_quota_reset_interval(&self) -> u64 {
            self.quota_reset_interval
        }

        #[ink(message)]
        pub fn get_quota_spent(&self) -> U256 {
            self.quota_spent
        }

        #[ink(message)]
        pub fn get_last_reset(&self) -> u64 {
            self.last_reset
        }
    }
}


