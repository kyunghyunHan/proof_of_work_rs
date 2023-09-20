use sha2::{Digest, Sha256};
use chrono::prelude::*;


pub struct Block {
  pub timestamp: i64,
  pub   data: Vec<u8>,
  pub   prev_block_hash: Vec<u8>,
  pub   hash: Vec<u8>,
}

 impl Block {
  pub   fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Block {
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

   pub fn set_hash(&mut self) {
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
