use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::vec::Vec;
use hex;
use proof_of_work_part1::Block;
use proof_of_work_part1::Blockchain;


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
