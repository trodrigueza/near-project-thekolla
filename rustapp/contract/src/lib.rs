use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, setup_alloc};
use near_sdk::{near_bindgen, Promise};
use near_sdk::collections::LookupMap;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KContract {
    records: LookupMap<String, String>,
}

impl Default for KContract {
    fn default() -> Self {
      Self {
        records: LookupMap::new(b"a".to_vec()),
      }
    }
  }

#[near_bindgen]
impl KContract {
    pub fn set_cid(&self, cid: String) {
        let account_id = env::signer_account_id();
        env::log(format!("{} just uploaded NTF Image with cid: {}", account_id, cid).as_bytes());
    }
    
      pub fn get_cid(&self, account_id:String) -> Option<String> {
      return self.records.get(&account_id);
   // env::state_read(self);
  }

    pub fn transaction(&self) {
        let amount: u128 = 500_000_000_000_000_000_000_000; // 0.5 $NEAR as yoctoNEAR
        let account_id = "thekolla.testnet".parse().unwrap();
        Promise::new(account_id).transfer(amount);
    }
}
