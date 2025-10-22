use sha2::{Sha256, Digest};
use hex;

pub fn sha256_digest (str : &String) -> String {
    let hash_str = str.to_string();
    let mut hasher = Sha256::new();
    hasher.update(hash_str.as_bytes());
    hex::encode(hasher.finalize())
}
