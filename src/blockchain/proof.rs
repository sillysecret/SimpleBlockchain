//export function isHashProofed ({ hash, difficulty = 4, prefix = '0' }: { hash: string, difficulty?: number, prefix?: string }) {
//    const check = prefix.repeat(difficulty)
//    return hash.startsWith(check)
//  }


pub fn isHashProofed(hash: String, difficulty: i32, prefix: String) -> bool {
    let check = prefix.repeat(difficulty as usize);
    hash.starts_with(&check)
}