use rust_chain::{block::Block, hashable::Hashable};

fn main() {
    let block = Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned());
    println!("{:?}", block.hash());
}
