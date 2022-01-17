use std::fmt::{self, Debug, Formatter};

use super::BlockHash;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block {{ index: {}, timestamp: {}, hash: {:?}, previous_hash: {:?}, nonce: {}, payload: {} }}", 
            &self.index,
            &self.timestamp,
            &hex::encode(&self.hash),
            &self.previous_hash,
            &self.nonce,
            &self.payload
        )
    }
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
