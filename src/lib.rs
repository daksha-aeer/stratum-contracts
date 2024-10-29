use std::fmt::format;
use sha3::{Digest, Keccak256};


// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, env, AccountId};

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
            salt: 12345
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

    fn calculate_proof(&self) -> Vec<u8> {
        let input = format!("{}{}", self.counter, self.salt);
        Keccak256::digest(input.as_bytes()).to_vec()
    }

    pub fn submit_proof(&mut self, proof: Vec<u8>) -> bool {
        let miner = env::signer_account_id(); // Get the wallet ID of the caller
        // Compare submitted proof with calculated proof
        if proof == self.calculate_proof() {
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
}
