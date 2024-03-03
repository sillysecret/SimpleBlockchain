pub fn isHashProofed(hash: String, difficulty: i32, prefix: String) -> bool {
    let check = prefix.repeat(difficulty as usize);
    hash.starts_with(&check)
}
