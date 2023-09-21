use std::fmt;
use std::vec::Vec;
use sha2::{Digest, Sha256};
use num_bigint::BigUint;
use num_traits::One;
use crate::block::{Block};
use crate::proof_of_work;
use std::cmp::PartialEq;

pub const MAX_NONCE: i64 = i64::MAX;
pub const TARGET_BITS: i64 = 20;

pub struct ProofOfWork<'a> {
 pub    block: &'a Block,
 pub   target: BigUint,
}

impl<'a> ProofOfWork<'a> {
   pub  fn new(block: &'a Block) -> ProofOfWork<'a> {
        let target = BigUint::one() << (256 - TARGET_BITS);
        ProofOfWork {
            block,
            target,
        }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.block.prev_block_hash);
        data.extend_from_slice(&self.block.data);
        data.extend_from_slice(&self.block.timestamp.to_be_bytes());
        data.extend_from_slice(&TARGET_BITS.to_be_bytes());
        data.extend_from_slice(&nonce.to_be_bytes());
        data
    }

    pub fn run(&self) -> (i64, Vec<u8>) {         
        let mut hash_int = BigUint::from_bytes_be(&[0; 32]);
        let mut hash = [0; 32];
        let mut nonce = 0;

        println!("Mining the block containing \"{}\"", String::from_utf8_lossy(&self.block.data));

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.update(&data);
            hash.copy_from_slice(&hasher.finalize());
              hash_int = BigUint::from_bytes_be(&hash);
              println!("{:?}",hash_int);

            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
            
        }

        (nonce, hash.to_vec())
    }

   pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize();

        let hash_int = BigUint::from_bytes_be(&hash);

        hash_int < self.target
    }
}

impl<'a> fmt::Debug for ProofOfWork<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProofOfWork{{ target: {:x} }}", self.target)
    }
}
