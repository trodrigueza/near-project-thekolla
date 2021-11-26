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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     // mock the context for testing, notice "signer_account_id" that was accessed above from env::
//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: "thekolla_testnet".to_string(),
//             signer_account_id: "tomasrodriguez_testnet".to_string(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: "tomasrodriguez_testnet".to_string(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 99_959_198_335_330_388_500_000_000,
//             account_locked_balance: 0,
//             storage_usage: 322_944,
//             attached_deposit: 500_000_000_000_000_000_000_000,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 19,
//         }
//     }

//     #[test]
//     fn transact() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = KContract::default();
//         assert_eq!(
//           500_000_000_000_000_000_000_000, context.attached_deposit
//         );
//     }
// }