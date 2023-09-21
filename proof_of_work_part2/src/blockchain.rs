use std::vec::Vec;
use crate::block::{Block};


pub struct Blockchain {
   pub  blocks: Vec<Block>,
}


impl Blockchain {
   pub  fn add_block(&mut self, data: Vec<u8>) {
    
        let prev_block_hash = self.blocks.last().map_or(Vec::new(), |block| block.hash.clone());
        let new_block = Block::new(data, prev_block_hash);
        self.blocks.push(new_block);
    }
    pub fn new_blockchain() -> Blockchain {
        let genesis_block = Block::new("Genesis Block".as_bytes().to_vec(), Vec::new());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
}

