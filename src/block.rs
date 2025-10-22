use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block{
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block  {
    pub fn new(index : u64, data : &str , previous_hash : &str ) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        Block {
            index : index,
            timestamp : timestamp,
            data : data.to_string(),
            previous_hash : previous_hash.to_string(),
            nonce : 0,
            hash : String::new(),
        }
    }
    pub fn calculate_hash(&self) -> String {
        let hash_str = format!("{} {} {} {} {}" ,self.index , self.timestamp , self.data , self.previous_hash , self.nonce );
        utils::sha256_digest(&hash_str)
    }
    pub fn mine(&mut self, difficulty: usize) {
        loop
        {            
            let hash = utils::sha256_digest(&(format!("{} {} {} {} {}" ,self.index , self.timestamp , self.data , self.previous_hash , self.nonce ).to_string()));
            if hash[..difficulty].as_bytes().iter().all(|x| *x == b'0') {
                self.hash = hash;
                break;
            }else {
                self.nonce += 1;
            }
        }
    }
}
