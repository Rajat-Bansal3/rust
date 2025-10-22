mod utils;
mod block;
mod blockchain;
mod config;

use blockchain::BlockChain;
use config::DIFFICULTY;

fn main() {
    let path = "blockchain.json";
    let mut chain: BlockChain;

    if let Some(loaded_chain) = BlockChain::load_from_file(path) {
        chain = loaded_chain;
    } else {
        chain = BlockChain::new(DIFFICULTY);
    }

    chain.add_block("Alice pays Bob 5 BTC".to_string());
    chain.add_block("Bob pays Charlie 2 BTC".to_string());

    println!("Chain valid? {}", chain.validate());
    println!("Total blocks: {}", chain.blocks.len());

    chain.save_to_file(&path);
}

