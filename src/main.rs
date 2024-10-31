use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u32,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().to_string();
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
        format!("{:x}", hasher.finalize())
    }
}
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        self.chain.push(new_block);
    }

    fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash
                != Block::calculate_hash(
                    current_block.index,
                    &current_block.timestamp,
                    &current_block.data,
                    &current_block.previous_hash,
                )
            {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First block after genesis".to_string());
    blockchain.add_block("Another block".to_string());

    println!("Blockchain: {:?}", blockchain);
    println!("Is blockchain valid? {}", blockchain.is_valid_chain());
}
