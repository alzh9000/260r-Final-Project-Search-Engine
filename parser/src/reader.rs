use nom::{bytes::complete::tag, complete::take, sequence::preceded, IResult};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn f() {
    // let file = File::open("/Volumes/SavvyT7Red/BitcoinCore/blocks/blk00000.dat")?;
    // let reader = BufReader::new(file);
}

fn hex_to_u32(input: &[u8; 4]) -> u32 {
    u32::from_le_bytes(*input)
}

pub fn raw_block_size(input: &[u8]) -> IResult<&[u8], u32> {
    // find magic byte sequence and then pull block size
    let magic: &[u8] = &[0xf9, 0xbe, 0xb4, 0xd9];
    let magic = tag(magic);
    preceded(magic, nom::number::complete::le_u32)(input)
}
