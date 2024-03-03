use crypto_hash::{hex_digest, Algorithm};
use std::convert::AsRef;

// A função hash em Rust
pub fn hash(data: impl AsRef<[u8]>) -> String {
    let data = data.as_ref();
    let hashed_data = hex_digest(Algorithm::SHA256, data);
    hashed_data
}

pub fn testhash(){
    let input_data = b"hello world";
        let hashed_data = hash(input_data);
        assert_eq!(hashed_data, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string());
}