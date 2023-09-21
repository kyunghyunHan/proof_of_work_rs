
/*작업증명
- 데이터를 추가하기위해서 어려운작업수행
- 이 작업이 블록체인을 안전하고 일관성 있게 만들어줌
- 

*/use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::vec::Vec;
use hex;
use proof_of_work_part2::Block;
use proof_of_work_part2::Blockchain;
use proof_of_work_part2::ProofOfWork;

fn main() {
    let mut  bc = Blockchain::new_blockchain();

    bc.add_block(Vec::from("Send 1 BTC to Ivan"));
    bc.add_block(Vec::from("Send 2 more BTC to Ivan"));

    for block in &bc.blocks {
        println!("Prev. hash: {:?}",hex::encode(&block.prev_block_hash));
        println!("Data: {:?}", String::from_utf8_lossy(&block.data));
        println!("Hash: {:?}\n", hex::encode(&block.hash));

        let pow =ProofOfWork::new(block);
        println!("Pow{}",ProofOfWork::validate(&pow));
    }
}
