mod blockchain;

use std::io::Chain;

use blockchain::block::Block;

use crate::blockchain::block;
fn main() {
    let mut blockchain = blockchain::Blockchain::new(4, "0".to_string());
    
    for i in 1..=10    {
        let block = blockchain.createPayload(format!("Block {}", i));
        
        let mine_block = match blockchain.mineblock(block){
            Ok(block) => block,
            Err(_) => break,
        }; 
        blockchain.pushblock(mine_block)
    
    }   
    print!("-----------------------BLOCKCHAIN-----------------------\n");
    blockchain.printblockchain();
    
}

