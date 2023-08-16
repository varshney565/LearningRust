#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod new_psp34 {

    use ink::codegen::{EmitEvent, Env};
    // imports from openbrush
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::extensions::burnable::*;
    use openbrush::contracts::psp34::extensions::metadata::*;
    // use openbrush::contracts::psp34::extensions::mintable::*;
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
            let mut token = self.owner_of(id.clone());
            let owner ;
            match token{
                Some(val)=>{ owner = val },
                None => {
                    return Err(PSP34Error::TokenNotExists);
                }
            }
            
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
            let res = self._mint_to(account, Id::U32(self.next_id));
            match(res){
                Ok(())=>{
                    self.set_token_uri(Id::U32(self.next_id), _token_uri);
                    self.next_id += 1;
                    return Ok(())
                },
                Err(err)=>{
                    return Err(err)
                }
            }
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
        fn mint_test_success(){
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            let mut contract = Contract::new();
            let uri: Vec<u8> = "SMAPLE_URI".into();

            //minting
            let mut result = contract.mint(accounts.bob, uri.clone());
            match(result){
                Ok(()) => {
                    //check if next id is equal to 1 or not.
                    assert_eq!(contract.next_id,1);
                    //check if the token_uri is "SAMPLE_URI or not"
                    let token_uri = contract.get_token_uri(Id::U32(0));
                    assert_eq!(token_uri.unwrap_or_default(),uri.clone());
                    //check if the token is minted to that address we have specified or not
                    let ow = contract.owner_of(Id::U32(0)).unwrap();
                    assert_eq!(ow,accounts.bob);
                },
                Err(err) => {
                    //in case of error
                    assert!(false,"Error while minting !!");
                    println!("Some error while minting !! --> {:?}",err);
                }
            }
        }

        #[ink::test]
        fn mint_test_success_only_owner_can_call(){
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut contract = Contract::new();
            let uri: Vec<u8> = "SMAPLE_URI".into();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);

            //minting
            let mut result = contract.mint(accounts.bob, uri.clone());
            match(result){
                Ok(()) => {assert!(false);},
                Err(err) => {
                    assert!(true);
                    println!("{:?}",err);
                }
            }
        }

        #[ink::test]
        fn mint_test_fail_token_already_there(){
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let uri : Vec<u8> = "SMAPLE_URI".into();
            //minting
            let mut result = contract.mint(accounts.alice, uri.clone());
            //error i am minting the same token again
            result = contract._mint_to(accounts.alice,Id::U32(0));
            match(result){
                Ok(()) => {},
                Err(err) => {
                    if let err = PSP34Error::TokenExists{
                        assert!(true);
                    }
                }
            }
        }

        #[ink::test] 
        fn burn_no_such_token(){
            let mut contract = Contract::new();
            let mock_account_id: AccountId = [0x42; 32].into();
            let k = contract.burn(mock_account_id,Id::U32(10));
            match(k){
                Ok(()) => {},
                Err(err) => {
                    assert_eq!(PSP34Error::TokenNotExists,err,"Some Unknown Error while burning !!");
                }
            }
        }

        #[ink :: test]
        fn burn_not_approved(){
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let uri : Vec<u8> = "SMAPLE_URI".into();
            let mut result = contract.mint(accounts.alice, uri.clone());
            match(result){
                Ok(())=>{
                    let ow = accounts.alice;
                    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
                    let res = contract.burn(ow,Id::U32(0));
                    match(res){
                        Ok(()) => {
                            assert!(false,"Everyone is able to burn !!");
                        },
                        Err(err) => {
                            assert!(true,"Not approved !!");
                        }
                    }
                },
                Err(err) => {
                    assert!(false,"Some Error while minting !!");
                    println!("Some Error while Minting !!");
                }
            }
        }

        #[ink::test]
        fn burn_success_owner_call(){
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let uri : Vec<u8> = "SMAPLE_URI".into();
            let mut result = contract.mint(accounts.alice, uri.clone());
            match(result){
                Ok(()) => {
                    let res = contract.burn(accounts.alice,Id::U32(0));
                    match(res){
                        Ok(()) => {
                            assert!(true,"Successful burn !!");
                        },
                        Err(err) => {
                            assert!(false,"Owner is not able to burn !!");
                            println!("{:?}",err);
                        }
                    }
                },
                Err(err) => {
                    assert!(false,"Error while minting !!");
                    println!("Error while minting,{:?} !!",err);
                }
            }
        }

        #[ink::test]
        fn burn_success_operator_call(){
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let uri : Vec<u8> = "SMAPLE_URI".into();
            let mut result = contract.mint(accounts.alice, uri.clone());
            match(result){
                Ok(()) => {
                    //give the access to bob on behalf of alice
                    let k = contract.approve(accounts.bob,Some(Id::U32(0)),true);
                    
                    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
                    let res = contract.burn(accounts.alice,Id::U32(0));
                    match(res){
                        Ok(()) => {
                            assert!(true);
                        },
                        Err(err) => {
                            assert!(false,"Operator is not able to burn !!");
                            println!("{:?}",err);
                        }
                    }
                },
                Err(err) => {
                    assert!(false,"Error while minting !!");
                    println!("Error while minting,{:?} !!",err);
                }
            }
        }

        #[ink :: test] 
        fn approve_test_success(){
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut result = contract.mint(accounts.bob, "URI".into());
            match(result){
                Ok(()) => {
                    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
                    let res = contract.approve(accounts.charlie,Some(Id::U32(0)),true);
                    match(res){
                        Ok(())=>{
                            assert!(contract.allowance(accounts.bob,accounts.charlie,Some(Id::U32(0))),"Error while approving !!");
                        },
                        Err(err)=>{
                            assert!(false,"Error while approving !!");
                            println!("{:?}",err);
                        }
                    }
                },
                Err(err) => {
                    assert!(false,"Some error while minting !!");
                    println!("{:?}",err);
                }
            }
        }

        #[ink :: test]
        fn approve_test_fail(){
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let result = contract.mint(accounts.bob, "URI".into());
            match(result){
                Ok(()) => {
                    let res = contract.approve(accounts.charlie,Some(Id::U32(0)),true);
                    match(res){
                        Ok(())=>{
                            assert!(false,"Error,Token owner is not approving but still getting approved !!");
                        },
                        Err(err)=>{
                            assert!(true);
                        }
                    }
                },
                Err(err) => {
                    assert!(false,"Some error while minting !!");
                    println!("{:?}",err);
                }
            }
        }

        #[ink :: test]
        fn get_token_uri_test() {
            let mut contract = Contract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut result = contract.mint(accounts.bob, "URI".into());
            match(result){
                Ok(()) => {
                    //case when token is present
                    let token = contract.get_token_uri(Id::U32(0));
                    match(token){
                        Some(t)=>{
                            assert!(true);
                        },
                        None=>{
                            assert!(false,"Error while fetching the token uri !");
                        }
                    }
                    //case when token is not present
                    let token = contract.get_token_uri(Id::U32(10));
                    match(token){
                        Some(t)=>{
                            assert!(false,"Getting URI of token which does't exist !!");
                        },
                        None=>{
                            assert!(true);
                        }
                    }
                },
                Err(err) => {
                    assert!(false,"Some error while minting !!");
                    println!("{:?}",err);
                }
            }
        }
    }
}