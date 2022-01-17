use rust_chain::block::Block;

fn main() {
    let block = Block::new(0, 0, vec![0; 32], 0, "Hello, world!".to_string());
    println!("{:?}", block);
}
