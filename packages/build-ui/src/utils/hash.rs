use sha3::Digest;

pub fn get_hash_string(bytes: &[u8]) -> String {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let hash = format!("{:x}", result);
    hash
}