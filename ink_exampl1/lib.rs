#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod demo {
    use ink::prelude::vec::Vec;
    #[ink(storage)]
    pub struct Demo {
        // Store some AccountId
        my_account: AccountId,
        // Store some Balance
        my_balance: Balance,
        // Store some Hash
        my_hash: Hash,
    }

    impl Demo {
        #[ink(constructor)]
        pub fn default() -> Self {
            let ac = [0x43;32].into();
            let ba = 1000;
            let ha = [0;32].into();
            Self {
                my_account : ac,
                my_balance : ba,
                my_hash : ha
            }
        }

        #[ink(constructor)]
        pub fn new(balance : u128) -> Self {
            Self {
                my_account : Self::env().caller(),
                my_balance : balance,
                my_hash : [0;32].into()
            }
        }

        #[ink(message)]
        pub fn get_account(&self) -> AccountId {
            self.my_account
        }

        #[ink(message)]
        pub fn get_hash(&self) -> Hash {
            self.my_hash
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.my_balance
        }
    }
}