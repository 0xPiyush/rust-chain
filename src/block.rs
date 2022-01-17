use super::BlockHash;

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        previous_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Block {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            previous_hash,
            nonce,
            payload,
        }
    }
}
