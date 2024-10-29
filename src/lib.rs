// use sha3::{Digest, Keccak256};
// use std::fmt::format;

// Find all our documentation at https://docs.near.org
use near_sdk::{env, log, near, AccountId};
use near_sdk_contract_tools::ft::*;
use std::borrow::Cow;

// Define the contract structure
#[derive(Default, FungibleToken)]
#[near(contract_state)]
pub struct Contract {
    counter: u64,
    salt: u64,
}

// Define the default, which automatically initializes the contract
// impl Default for Contract {
//     fn default() -> Self {
//         Self {
//             counter: 0,
//             salt: 12345,
//         }
//     }
// }

// Implement the contract structure
#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {
            counter: 0,
            salt: 12345,
        };
        contract.set_metadata(&ContractMetadata::new("Stratum", "STR", 0));

        contract
    }
    pub fn get_salt(&self) -> u64 {
        self.salt
    }

    pub fn get_counter(&self) -> u64 {
        self.counter
    }

    // fn calculate_proof(&self) -> Vec<u8> {
    //     let input = format!("{}{}", self.counter, self.salt);
    //     Keccak256::digest(input.as_bytes()).to_vec()
    // }

    fn calculate_proof_v2(&self) -> Vec<u8> {
        env::keccak256(&self.counter.to_le_bytes())
    }

    pub fn submit_proof(&mut self, proof: Vec<u8>) -> bool {
        let miner = env::signer_account_id();
        let expected_proof = self.calculate_proof_v2();
        env::log_str(&format!("Expected proof {:?}", expected_proof));

        if proof == expected_proof {
            self.counter += 1; // Increment the counter for the next challenge
            env::log_str(&format!("Proof validated for miner: {}", miner));

            self.storage_deposit(Some(miner.clone()), Some(true));

            self.mint(&Nep141Mint {
                amount: 1,
                receiver_id: Cow::Borrowed(&miner),
                memo: None,
            })
            .unwrap();

            true
        } else {
            env::log_str("Proof validation failed.");
            false
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Digest, Keccak256};

    #[test]
    fn get_hash() {
        let counter: u64 = 0;

        let hash = Keccak256::digest(counter.to_le_bytes()).to_vec();
        println!("hash {:?}", hash);
    }
}
