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
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    pub enum Error {
        NotDelegate,
        NotOwner,
        TransferFailed,
    }

    impl CardContract {
        #[ink(constructor)]
        pub fn new(delegate: H160) -> Self {
            Self {
                owner: Self::env().caller(),
                delegate,
            }
        }

        #[ink(message)]
        pub fn pay(
            &mut self,
            intermediary_contract: H160,
            amount: U256,
        ) -> Result<(), Error> {
            if self.env().caller() != self.delegate {
                return Err(Error::NotDelegate);
            }

            // ink! 6.0 change: Use Selector::new with a byte array.
            // 0x636F696E is the hex for the ASCII string "coin".
            let selector_bytes = [0x63, 0x6F, 0x69, 0x6E];

            build_call::<DefaultEnvironment>()
                .call(intermediary_contract)
                .transferred_value(amount)
                .exec_input(ExecutionInput::new(Selector::new(selector_bytes)))
                .returns::<()>()
                .invoke();

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
        pub fn get_delegate(&self) -> H160 {
            self.delegate
        }
    }
}


