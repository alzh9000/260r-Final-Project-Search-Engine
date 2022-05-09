use nom::{bytes::complete::tag, complete::take, sequence::preceded, IResult};

pub fn f() -> u32 {
    let file = std::fs::read("/Volumes/SavvyT7Red/BitcoinCore/blocks/blk00000.dat").unwrap();
    let file = file.as_slice();
    let (_input, size) = raw_block_size(file).unwrap();
    size
}

pub fn raw_block_size(input: &[u8]) -> IResult<&[u8], u32> {
    // find magic byte sequence and then pull block size
    let magic: &[u8] = &[0xf9, 0xbe, 0xb4, 0xd9];
    let magic = tag(magic);
    preceded(magic, nom::number::complete::le_u32)(input)
}
