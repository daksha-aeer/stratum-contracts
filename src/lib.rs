// use sha3::{Digest, Keccak256};
// use std::fmt::format;

// Find all our documentation at https://docs.near.org
use near_sdk::{env, log, near, AccountId};

// Define the contract structure
#[near(contract_state)]
pub struct Contract {
    counter: u64,
    salt: u64,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            counter: 0,
            salt: 12345,
        }
    }
}

// Implement the contract structure
#[near]
impl Contract {
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
        let miner = env::signer_account_id(); // Get the wallet ID of the caller
                                              // Compare submitted proof with calculated proof

        let expected_proof = self.calculate_proof_v2();
        env::log_str(&format!("Expected proof {:?}", expected_proof));

        if proof == expected_proof {
            self.counter += 1; // Increment the counter for the next challenge
            env::log_str(&format!("Proof validated for miner: {}", miner));
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
