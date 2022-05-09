type Hash256 = sha2::digest::generic_array::GenericArray<u8, sha2::digest::consts::U16>;

pub struct Metadata {
    id: Hash256,
    block: Hash256,
    blockheight: u32,
    size: u32,
    total_input: u64,  // in Satoshi (1/100M of a Bitcoin)
    total_output: u64, // in Satoshi (1/100M of a Bitcoin)
    fees: u64,         // in Satoshi (1/100M of a Bitcoin)
}

pub struct Block {
    id: Hash256,
    version: u32,
    prev_block_id: Hash256,
    hash_merkle_root: Hash256,
    unix_time: u32,
    tx_count: u32,
}

// TODO: raw versions of these?
