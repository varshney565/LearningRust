#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod new_psp34 {

    use ink::codegen::{EmitEvent, Env};
    // imports from openbrush
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::extensions::burnable::*;
    use openbrush::contracts::psp34::extensions::metadata::*;
    use openbrush::contracts::psp34::extensions::mintable::*;
    use openbrush::storage::Mapping;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,

        // Fields of current contract
        /// mapping from token id to `token_uri`
        token_uris: Mapping<Id, String>,

        /// A unique identifier for the tokens which have been minted (and are therefore
        /// supported) by this contract.
        next_id: u32,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    /// Event emitted when a set_token_uri occurs.
    #[ink(event)]
    pub struct SetTokenUri {
        #[ink(topic)]
        _id: Id,
        #[ink(topic)]
        _token_uri: String,
    }

    // Override event emission methods
    impl psp34::Internal for Contract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });
        }
    }

    // Section contains default implementation without any modifications
    impl PSP34 for Contract {}
    impl PSP34Metadata for Contract {}
    impl Ownable for Contract {}

    impl PSP34Burnable for Contract {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let owner = self.owner_of(id.clone()).unwrap();
            let caller = self.env().caller();

            if owner != caller && !self._allowance(&owner, &caller, &Some(&id)) {
                return Err(PSP34Error::NotApproved);
            }
            self.remove_token_uri(id.clone());
            self._burn_from(account, id)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();

            _instance._init_with_owner(_instance.env().caller());
            let collection_id = _instance.collection_id();
            _instance._set_attribute(
                collection_id.clone(),
                String::from("name"),
                String::from("MyPSP34"),
            );
            _instance._set_attribute(collection_id, String::from("symbol"), String::from("MPSP"));
            _instance
        }

        pub fn _emit_set_token_uri_event(&self, _id: Id, _token_uri: String) {
            self.env().emit_event(SetTokenUri { _id, _token_uri });
        }
        pub fn _emit_updated_token_uri_event(&self, _id: Id, _token_uri: String) {
            self.env().emit_event(SetTokenUri { _id, _token_uri });
        }
        pub fn _emit_remove_token_uri_event(&self, _id: Id) {}

        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn mint(&mut self, account: AccountId, _token_uri: String) -> Result<(), PSP34Error> {
            self.set_token_uri(Id::U32(self.next_id), _token_uri);
            self._mint_to(account, Id::U32(self.next_id));
            self.next_id += 1;
            Ok(())
        }

        fn set_token_uri(&mut self, id: Id, _token_uri: String) -> Result<(), PSP34Error> {
            self.token_uris.insert(&id, &_token_uri);
            self._emit_set_token_uri_event(id, _token_uri);

            Ok(())
        }

        #[ink(message)]
        pub fn get_token_uri(&self, id: Id) -> Option<String> {
            self.token_uris.get(&id)
        }

        fn remove_token_uri(&mut self, id: Id) -> Result<(), PSP34Error> {
            self.token_uris.remove(&id);
            self._emit_remove_token_uri_event(id);

            Ok(())
        }

        /// Modifies the code which is used to execute calls to this contract address (`AccountId`).
        ///
        /// We use this to upgrade the contract logic. We don't do any authorization here, any caller
        /// can execute this method. In a production contract you would do some authorization here.
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), PSP34Error> {
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests{
        use ink::primitives::AccountId;

        use super::*;
        #[ink::test]
        fn burn_test1(){
            let mut contract = Contract::new();
            let mock_account_id: AccountId = [0x42; 32].into();
            //assuming mint is working perfectly fine !!
            contract.mint(mock_account_id, "SMAPLE_URI")
            .expect("Can mint");
            //three cases
            //first --> 

            //second --> 
        }
    }
}


