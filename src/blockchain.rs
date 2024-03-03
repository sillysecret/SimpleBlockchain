mod block;
mod hash;

use block::{Block,Payload,Header};
use time::OffsetDateTime;
use hash::{hash,testhash};



pub struct Blockchain {
    blocks: Vec<Block>,
    powPrefix: char,
    difficulty: i32,
}



impl Blockchain{
    pub fn new(difficulty: i32, powPrefix:char) -> Self {
        let mut chain = Blockchain {
            blocks: Vec::new(),
            powPrefix : powPrefix,
            difficulty: difficulty,
        };

        chain.create_genesis_block();
        
        chain
    }

    fn create_genesis_block(&mut self) {
        let offsetdate = OffsetDateTime::now_utc().date();  
        let payload = Payload::new(0,offsetdate,"Genesis Block".to_string(), "0".to_string());
        let header = Header::new(0, hash(serde_json::to_string(&payload).unwrap().as_bytes()));
        let block = Block::new(header, payload);
        self.blocks.push(block);
    }

    pub fn show_blocks(&self) {
        for block in &self.blocks {
            println!("Block: {:?}", serde_json::to_string(&block).unwrap());
        }
    }

    pub fn testahash(&self){
        hash::testhash();
    }



}










