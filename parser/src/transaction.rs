use duplicate::duplicate;
use itertools::Itertools;
use sha2::Digest;
use std::fmt;
use std::string::String;

// Note that Hash256 types are stored internally in little-endian! (reverse byte order). This is
// due to an accident in the original Bitcoin protocol, and we keep it this way for efficiency
// reasons.
type Hash256 = [u8; 32];

// Define distinct hash types to avoid mixing them up accidentally. But keep the same
// implementations of all three.
duplicate! {
    [ name; [TxHash]; [BlockHash]; [MerkleRoot] ]
#[derive(Clone, Copy)]
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

#[derive(Debug)]
pub struct Metadata {
    pub id: TxHash,
    pub version: u32,
    pub block: BlockHash,
    pub blockheight: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct Block {
    pub id: BlockHash,
    pub version: u32,
    pub prev_block_id: BlockHash,
    pub merkle_root: MerkleRoot,
    pub unix_time: u32,
    pub tx_count: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Output {
    pub index: u32,
    pub value: Value,
}

#[derive(Debug)]
pub struct Input {
    pub source_tx: TxHash,
    pub source_index: u32,
    pub value: Option<Value>, // we only store value here as an optimization. It might not be known at the time that we parse the given block, so we use an option. At the end of parsing the data, there should be no inputs with None values.
}

// TODO: move somewhere else
pub fn hash_twice(x: &[u8]) -> Hash256 {
    let once = sha2::Sha256::digest(x);
    sha2::Sha256::digest(once).into()
}

pub fn hash_once(x: &[u8]) -> Hash256 {
    sha2::Sha256::digest(x).into()
}
