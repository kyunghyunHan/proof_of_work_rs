use std::time::{SystemTime, UNIX_EPOCH};
use num_bigint::BigUint;
use num_traits::One;

use crate::{proof_of_work::{self,TARGET_BITS}, ProofOfWork};
// Block keeps block headers
pub struct Block {
  pub  timestamp: i64,
  pub data: Vec<u8>,
  pub prev_block_hash: Vec<u8>,
  pub hash: Vec<u8>,
  pub nonce: i64,
}

impl Block {
    // NewBlock creates and returns Block
    pub fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;
        let mut block = Block {
            timestamp,
            data: data.to_vec(),
            prev_block_hash,
            hash: Vec::new(),
            nonce: 0,
        };
        let target = BigUint::one() << (256 - TARGET_BITS);

        let proof_of_work=
            proof_of_work::ProofOfWork{
                block:&block,
                target
            };
        
        let (nonce, hash) = ProofOfWork::run(&proof_of_work);
        block.hash = hash;
        block.nonce = nonce as i64;
        block
    }
}

// NewGenesisBlock creates and returns genesis Block
fn new_genesis_block() -> Block {
    Block::new("Genesis Block".as_bytes().to_vec(), Vec::new())
}

