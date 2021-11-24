use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, setup_alloc};
use near_sdk::{near_bindgen, Promise};
setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KContract {}


#[near_bindgen]
impl KContract {
    pub fn set_cid(&self, cid: String) {
        let account_id = env::signer_account_id();
        env::log(format!("{} just uploaded NTF Image with cid: {}", account_id, cid).as_bytes());
    }

    // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
    // self.records.get(&account_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn transaction(&self) {
        let amount: u128 = 500_000_000_000_000_000_000_000; // 0.5 $NEAR as yoctoNEAR
        let account_id = "thekolla.testnet".parse().unwrap();
        Promise::new(account_id).transfer(amount);
    }
}
