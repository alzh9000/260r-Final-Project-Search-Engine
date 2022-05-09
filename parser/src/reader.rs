use crate::transaction;
use crate::transaction::print_hash;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u32,
    sequence::{preceded, tuple},
    IResult,
};
use nom_varint::take_varint;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Matcher {
    unmatched_inputs: HashMap<transaction::Hash256, transaction::Input>,
    unmatched_outputs: HashMap<transaction::Hash256, transaction::Output>,
}

impl Matcher {
    fn new() -> Matcher {
        Matcher {
            unmatched_inputs: HashMap::new(),
            unmatched_outputs: HashMap::new(),
        }
    }
}

pub fn f() -> u32 {
    let mut matcher = Matcher::new();
    let file = std::fs::read("/Volumes/SavvyT7Red/BitcoinCore/blocks/blk00000.dat").unwrap();
    let file = file.as_slice();
    let (input, size) = raw_block_size(file).unwrap();
    let (_input, block) = parse_block_header_and_tx_count(input).unwrap();

    println!("{:?}", block);

    println!("Block ID: {}", print_hash(&block.id));
    println!("Block previous ID: {}", print_hash(&block.prev_block_id));
    println!("Block Merkle root: {}", print_hash(&block.merkle_root));
    size
}

fn raw_block_size(input: &[u8]) -> IResult<&[u8], u32> {
    // find magic byte sequence and then pull block size
    let magic: &[u8] = &[0xf9, 0xbe, 0xb4, 0xd9];
    let magic = tag(magic);
    preceded(magic, le_u32)(input)
}

fn take_32_bytes_as_hash(input: &[u8]) -> IResult<&[u8], crate::transaction::Hash256> {
    let (input, data) = take(32u8)(input)?;
    let res: crate::transaction::Hash256 = data.try_into().expect("Wrong length; expected 32");
    Ok((input, res))
}

// Note that height is not correct when this function returns.
fn parse_block_header_and_tx_count(input: &[u8]) -> IResult<&[u8], transaction::Block> {
    let (input, header) = take(80u8)(input)?;

    // hash entire header to get the block ID
    let id = transaction::hash_twice(header);

    let mut parser = tuple((le_u32, take_32_bytes_as_hash, take_32_bytes_as_hash, le_u32));
    let (_, (version, prev_id, merkle_root, unix_time)) = parser(header)?;
    println!("{:?}", &input[..10]);
    let (input, tx_count) = take_varint(input)?;

    Ok((
        input,
        transaction::Block {
            id,
            version,
            prev_block_id: prev_id,
            merkle_root,
            unix_time,
            tx_count: tx_count.try_into().unwrap(),
            height: u32::MAX,
        },
    ))
}

fn parse_transaction(input: &[u8]) -> IResult<&[u8], transaction::Metadata> {
    let (input, version) = le_u32(input)?;
    let (input, mut input_count) = take_varint(input)?;

    // Need to deal with the optional witness flag in newer protocols versions if it's there. Note
    // that we don't use nom::combinator::cond to avoid polluting our computed values with None
    // values.
    let witnesses_enabled = input_count == 0;
    if (witnesses_enabled) {
        let (input, _) = take(1u8)(input)?;
        let (input, input_count) = take_varint(input)?;
    } else {
        // already have the correct input count
    };
}

fn take_tx_input(input: &[u8]) -> IResult<&[u8], transaction::Metadata> {}
