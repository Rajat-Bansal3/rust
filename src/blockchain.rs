use crate::block::Block;
use serde::{Serialize, Deserialize};
use std::fs;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockChain{
    pub blocks : Vec<Block>,
    pub difficulty : usize
}

impl BlockChain {
    pub fn new(difficulty : usize)-> BlockChain {
        let data = String::from ("First Block");
        let mut block = Block::new(0 , &data , "0" );
        block.mine(difficulty);
        BlockChain{
            blocks : vec![block],
            difficulty : difficulty 
        }
    }
    pub fn add_block (&mut self , data : String) {
        let prev_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(prev_block.index + 1 , &data , &prev_block.hash);
        new_block.mine(self.difficulty);
        self.blocks.push(new_block);
    }
    pub fn validate(&self) -> bool {
        let len = self.blocks.len();
        for i in 1..len{
            let curr_block = &self.blocks[i];
            let prev_block = &self.blocks[i - 1];
            if prev_block.hash != curr_block.previous_hash {
                return false;
            }
            if curr_block.hash != curr_block.calculate_hash() {
                return false;
            }
        }
        true
    }
    pub fn save_to_file(&self , path : &str) {
        let sdata = serde_json::to_string_pretty(&self).expect("Failed to serialize blockchain");
        fs::write(path , sdata).expect("Failed to write blockchain to file");
    }
    pub fn load_from_file(path: &str) -> Option<BlockChain> {
        if let Ok(data) = fs::read_to_string(path) {
            if let Ok(chain) = serde_json::from_str::<BlockChain>(&data) {
                return Some(chain);
            }
        }
        None
    }
}
