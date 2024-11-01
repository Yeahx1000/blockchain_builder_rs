use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String, difficulty: u32) -> Block {
        let timestamp = Utc::now().to_string();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine_block(difficulty);
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);
        while &self.hash[..difficulty as usize] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

#[derive(serde::Deserialize, Serialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Blockchain {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Error> {
        let data = serde_json::to_string(&self)?;
        fs::write(filename, data).expect("Unable to write file");
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Blockchain, Error> {
        if Path::new(filename).exists() {
            let data = fs::read_to_string(filename).expect("Unable to read file");
            let blockchain: Blockchain = serde_json::from_str(&data)?;
            Ok(blockchain)
        } else {
            Ok(Blockchain::new(2))
        }
    }
}
