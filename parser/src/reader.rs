use crate::transaction;
use crate::transaction::print_hash;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u32,
    sequence::{preceded, tuple},
    IResult,
};

pub fn f() -> u32 {
    let file = std::fs::read("/Volumes/SavvyT7Red/BitcoinCore/blocks/blk00000.dat").unwrap();
    let file = file.as_slice();
    let (input, size) = raw_block_size(file).unwrap();
    let (_input, header) = take_header(input).unwrap();
    let (_input, block) = parse_block_header(header).unwrap();

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

fn take_header(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(80u8)(input)
}

fn take_32_bytes_as_hash(input: &[u8]) -> IResult<&[u8], crate::transaction::Hash256> {
    let (input, data) = take(32u8)(input)?;
    let res: crate::transaction::Hash256 = data.try_into().expect("Wrong length; expected 32");
    Ok((input, res))
}

// Note that tx_count and height are not correct when this function returns.
fn parse_block_header(input: &[u8]) -> IResult<&[u8], transaction::Block> {
    assert_eq!(input.len(), 80);

    // hash entire header to get the block ID
    let id = transaction::hash_twice(input);

    let mut parser = tuple((le_u32, take_32_bytes_as_hash, take_32_bytes_as_hash, le_u32));
    let (input, (version, prev_id, merkle_root, unix_time)) = parser(input)?;

    Ok((
        input,
        transaction::Block {
            id,
            version,
            prev_block_id: prev_id,
            merkle_root,
            unix_time,
            tx_count: u32::MAX,
            height: u32::MAX,
        },
    ))
}
