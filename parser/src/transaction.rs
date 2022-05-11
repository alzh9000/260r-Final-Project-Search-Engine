use duplicate::duplicate;
use itertools::Itertools;
use std::fmt;
use std::hash::Hash;
use std::string::String;

// Note that Hash256 types are stored internally in little-endian! (reverse byte order). This is
// due to an accident in the original Bitcoin protocol, and we keep it this way for efficiency
// reasons.
pub type Hash256 = [u8; 32];

// Define distinct hash types to avoid mixing them up accidentally. But keep the same
// implementations of all three.
duplicate! {
    [ name; [TxHash]; [BlockHash]; [MerkleRoot] ]
#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct name(Hash256);

impl fmt::Debug for name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_hash(&self.0))
    }
}

impl name {
    pub fn new(data: [u8; 32]) -> name {
        name{0: data}
    }
}

impl std::convert::From<[u8; 32]> for name {
    fn from(data: [u8; 32]) -> name {
        name{ 0: data }
    }
}

impl std::convert::AsRef<[u8; 32]> for name {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

}

fn print_hash(h: &Hash256) -> String {
    // Since these are stored in reverse byte order, we need to iterate backwards.
    format!("{:02x}", h.iter().rev().format(""))
}

// Value is always denominated in Satoshis. (1e-8 BTC)
pub type Value = u64;

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub id: TxHash,
    pub version: u32,
    pub block: BlockHash,
    pub block_height: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub id: BlockHash,
    pub version: u32,
    pub prev_block_id: BlockHash,
    pub merkle_root: MerkleRoot,
    pub unix_time: u32,
    pub tx_count: u32,
    pub height: u32,
}

// We define the following three struct types to denote inputs and outputs of Bitcoin transactions.

#[derive(Debug)]
// An InputOutputPair is a "link" between two transactions. `source` is the parent transaction, and
// `dest` is the child. Note that source must exist, but dest might not (if the relevant output is
// unspent).
pub struct InputOutputPair {
    pub source: Output,
    pub dest: Option<Input>,
}

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub src_tx: TxHash,
    pub src_index: u32,
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub dest_tx: TxHash,
    pub dest_index: u32,
}