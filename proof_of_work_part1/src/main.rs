use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::vec::Vec;
use hex;

// Block structure
struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            timestamp,
            data,
            prev_block_hash,
            hash: Vec::new(),
        };
        block.set_hash();
        block
    }

    fn set_hash(&mut self) {
        let timestamp_bytes = self.timestamp.to_string().into_bytes();
        let mut headers = Vec::new();
        headers.extend_from_slice(&self.prev_block_hash);
        headers.extend_from_slice(&self.data);
        headers.extend_from_slice(&timestamp_bytes);
        
        let mut hasher = Sha256::new();
        hasher.update(&headers);
        self.hash = hasher.finalize().to_vec();
    }
}

// Blockchain structure
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(Vec::from("Genesis Block"), Vec::new());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: Vec<u8>) {
        let prev_block_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(data, prev_block_hash);
        self.blocks.push(new_block);
    }
}

fn main() {
    let mut bc = Blockchain::new();

    bc.add_block(Vec::from("Send 1 BTC to Ivan"));
    bc.add_block(Vec::from("Send 2 more BTC to Ivan"));

    for block in &bc.blocks {
        println!("Prev. hash: {:?}",hex::encode(&block.prev_block_hash));
        println!("Data: {:?}", String::from_utf8_lossy(&block.data));
        println!("Hash: {:?}\n", hex::encode(&block.hash));
    }
}
