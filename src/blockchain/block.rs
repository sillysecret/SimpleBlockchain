
use time::Date;
use serde::Serialize;
time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");


#[derive(Serialize)]
pub struct Header {
    nonce: i32,
    block_hash: String,
}

impl Header {
    // Método construtor para Header
    pub fn new(nonce: i32, block_hash: String) -> Self {
        Header {
            nonce,
            block_hash,
        }
    }
}

#[derive(Serialize)]
pub struct Payload {
    sequence: i32,
    timestamp: Date,
    data: String,
    previous_hash: String,
}


impl Payload {
    // Método construtor para Payload
    pub fn new(sequence: i32,timestamp: Date, data: String, previous_hash: String) -> Self {
        
        
        Payload {
            sequence,
            timestamp,
            data,
            previous_hash,
        }
    }
}

#[derive(Serialize)]
pub struct Block {
    header: Header,
    payload: Payload,
}

impl Block {
    // Método construtor para Block
    pub fn new(header: Header, payload: Payload) -> Self {
        Block {
            header,
            payload,
        }
    }
}