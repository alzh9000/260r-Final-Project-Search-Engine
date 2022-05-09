use itertools::Itertools;
use std::string::String;

use sha2::Digest;

// Note that Hash256 types are stored internally in little-endian! (reverse byte order). This is
// due to an accident in the original Bitcoin protocol, and we keep it this way for efficiency
// reasons.
pub type Hash256 = [u8; 32];

pub fn print_hash(h: &Hash256) -> String {
    // Since these are stored in reverse byte order, we need to iterate backwards.
    format!("{:02x}", h.iter().rev().format(""))
}

#[derive(Debug)]
pub struct Metadata {
    pub id: Hash256,
    pub block: Hash256,
    pub blockheight: u32,
    pub size: u32,
    pub total_input: u64,  // in Satoshi (1/100M of a Bitcoin)
    pub total_output: u64, // in Satoshi (1/100M of a Bitcoin)
    pub fees: u64,         // in Satoshi (1/100M of a Bitcoin)
}

#[derive(Debug)]
pub struct Block {
    pub id: Hash256,
    pub version: u32,
    pub prev_block_id: Hash256,
    pub merkle_root: Hash256,
    pub unix_time: u32,
    pub tx_count: u32,
}

// TODO: move somewhere else
pub fn hash_twice(x: &[u8]) -> Hash256 {
    let once = sha2::Sha256::digest(x);
    sha2::Sha256::digest(once).into()
}
