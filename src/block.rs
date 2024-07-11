use serde_derive::Serialize;

use crate::transaction::Transaction;

#[derive(Serialize, Debug)]
pub struct BlockHeader {
    pub timestamp: u64,
    pub nonce: u32,
    pub prev_hash: String,
    pub merkle: String,
    pub difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub count: u32,
    pub transactions: Vec<Transaction>,
}