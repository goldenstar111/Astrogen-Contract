use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{ LookupMap};
use near_sdk::json_types::{ U128 };
use near_sdk::{
    env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey, Balance
};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //keeps track of all the token IDs for a given account
    pub user_info: LookupMap<AccountId, Balance>,
	
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshStorageKey,BorshSerialize)]
pub enum StorageKey {
    UserInfo,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        //create a variable of type Self with all the fields initialized. 
        Self {
			user_info: LookupMap::new(StorageKey::UserInfo),
        }
    }
	
	pub fn get_amount_by_owner(&self, account_id: AccountId ) -> U128{
		let tokens = self.user_info.get(&account_id);
		if let Some(tokens) = tokens {
			U128(tokens)
		} else {
			U128(0)
		}
    }
	
	#[payable]
	pub fn deposit_storage(&mut self, account_id: AccountId) {

		let attached_deposit = env::attached_deposit();
		
		self.user_info.insert(&account_id, &attached_deposit);
		
    }

}