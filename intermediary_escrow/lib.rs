#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract(env = ink::env::DefaultEnvironment)]
pub mod intermediary_escrow {
    use ink::primitives::H160;
    use ink::primitives::U256;
    use parity_scale_codec::{Encode, Decode};
    use scale_info::TypeInfo;

    #[ink(storage)]
    pub struct IntermediaryEscrow {
        admin: H160,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    pub enum Error {
        NotAdmin,
    }

    impl IntermediaryEscrow {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                admin: Self::env().caller(),
            }
        }

        /// Receive payment (payable, selector must match the caller's expectation)
        #[ink(message, payable, selector = 0x636F696E)]
        pub fn receive_payment(&mut self) {
            // Accepts payment. Add logic if needed.
        }

        #[ink(message)]
        pub fn get_admin(&self) -> H160 {
            self.admin
        }
    }
} 