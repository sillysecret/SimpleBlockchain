pub mod block;
mod hash;
mod proof;


use block::{Block,Payload,Header};
use time::OffsetDateTime;
use hash::hash;
use proof::isHashProofed;
use std::time::SystemTime;

pub struct Blockchain {
    blocks: Vec<Block>,
    powPrefix: String,
    difficulty: i32,
}



impl Blockchain{
    pub fn new(difficulty: i32, powPrefix:String) -> Self {
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

    fn getlastblock(&self) -> &Block {
        self.blocks.last().unwrap()
    }

    fn getPreviusblockhash(&self) -> String {
        self.getlastblock().header.block_hash.clone()
    }

    pub fn createPayload(&mut self ,data: String) -> Payload{
        let offsetdate = OffsetDateTime::now_utc().date();  
        Payload::new(self.getlastblock().payload.sequence + 1, offsetdate, data, self.getPreviusblockhash())
    }

    pub fn mineblock(&mut self,Payload : Payload) -> Result<block::Block,()>{
        let mut nonce = 0;
        let startTime = SystemTime::now();

        while true {
            let blockhash = hash(serde_json::to_string(&Payload).unwrap().as_bytes());
            let profinghash = hash(format!("{}{}", blockhash, nonce).to_string().as_bytes());
            
            if isHashProofed(profinghash,self.difficulty,self.powPrefix.clone()) {
                let endTime = SystemTime::now();
                let shortHash = match blockhash.get(0..12){
                    Some(shortHash) => shortHash,
                    None => {
                        // Lidar com erro, se o tempo de término for antes do tempo de início
                        println!("Erro: Não foi possível obter os primeiros 12 caracteres do hash.");
                        return Err(());
                    }
                };
                let mineTime = match endTime.duration_since(startTime) {
                    Ok(duration) => duration,
                    Err(_) => {
                        // Lidar com erro, se o tempo de término for antes do tempo de início
                        println!("Erro: O tempo de término é antes do tempo de início.");
                        return Err(());
                    }
                };

                
                println!("Bloco minerado: {} em {} milisegundos | tentativas ou nonces {}" , shortHash, mineTime.as_millis(),nonce);
                return Ok(Block::new(Header::new(nonce,blockhash),Payload))

            }
            nonce+=1 ;
            
        }
        Err(())
    }

    fn veryfyblock(&self,block: &Block) -> bool{
        if (block.payload.previous_hash != self.getlastblock().header.block_hash) {
            println!("Erro: Bloco não é válido");
            return false;
        }

        if(!isHashProofed(hash(format!("{}{}",hash(serde_json::to_string(&block.payload).unwrap().as_bytes()),block.header.nonce)),self.difficulty,self.powPrefix.clone())){
            println!("Erro: Bloco não é válido");
            return false;            
        }

        true
    }

    pub fn pushblock(&mut self,block: Block){
        if(self.veryfyblock(&block)){
            self.blocks.push(block);    
        }
    }

    pub fn printblockchain(self){
        for block in self.blocks {
            println!("Bloco: {:?}", block);
        }
    }
}













