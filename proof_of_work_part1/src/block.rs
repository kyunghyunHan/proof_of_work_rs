use crate::Block;


pub struct Blockchain {
  pub blocks: Vec<Block>,
}

 impl Blockchain {
    pub  fn new() -> Blockchain {
        let genesis_block = Block::new(Vec::from("Genesis Block"), Vec::new());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

   pub  fn add_block(&mut self, data: Vec<u8>) {
        let prev_block_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = Block::new(data, prev_block_hash);
        self.blocks.push(new_block);
    }
}

