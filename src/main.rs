mod blockchain;

fn main() {
    let mut blockchain = blockchain::Blockchain::new(4, '0');
    blockchain.show_blocks();
    blockchain.testahash();
}

