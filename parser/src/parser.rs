use crate::transaction;
use nom::{
    bytes::complete::{tag, take},
    combinator::cond,
    number::complete::{le_u32, le_u64},
    sequence::{preceded, tuple},
    IResult, ToUsize,
};
use nom_varint::take_varint;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
    unmatched_inputs: HashMap<transaction::TxHash, Vec<transaction::Input>>,
    unmatched_outputs: HashMap<transaction::TxHash, Vec<transaction::Input>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            unmatched_inputs: HashMap::new(),
            unmatched_outputs: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
        let file = std::fs::read("/Volumes/SavvyT7Red/BitcoinCore/blocks/blk00000.dat").unwrap();
        let file = file.as_slice();

        let input = file;
        loop {
            let (input, _size) = match raw_block_size(input) {
                Err(e) => return,
                Ok((i, s)) => (i, s),
            };
            let (input, block) = parse_block_header_and_tx_count(input).unwrap();

            println!("{:#?}", block);

            let (input, txs) =
                parse_transactions(input, block.id, block.tx_count.to_usize()).unwrap();

            println!("{:#?}", &txs);
        }
    }
}

fn raw_block_size(input: &[u8]) -> IResult<&[u8], u32> {
    // find magic byte sequence and then pull block size
    let magic: &[u8] = &[0xf9, 0xbe, 0xb4, 0xd9];
    let magic = tag(magic);
    preceded(magic, le_u32)(input)
}

fn take_32_bytes_as_hash(input: &[u8]) -> IResult<&[u8], [u8; 32]> {
    let (input, data) = take(32u8)(input)?;
    let res: [u8; 32] = data.try_into().expect("Wrong length; expected 32");
    Ok((input, res))
}

// Note that height is not correct when this function returns.
fn parse_block_header_and_tx_count(input: &[u8]) -> IResult<&[u8], transaction::Block> {
    let (input, header) = take(80u8)(input)?;

    // hash entire header to get the block ID
    let id = transaction::hash_twice(header);

    let mut parser = tuple((le_u32, take_32_bytes_as_hash, take_32_bytes_as_hash, le_u32));
    let (_, (version, prev_id, merkle_root, unix_time)) = parser(header)?;
    let (input, tx_count) = take_varint(input)?;

    Ok((
        input,
        transaction::Block {
            id: id.into(),
            version,
            prev_block_id: prev_id.into(),
            merkle_root: merkle_root.into(),
            unix_time,
            tx_count: tx_count.try_into().unwrap(),
            height: u32::MAX,
        },
    ))
}

fn parse_transactions(
    input: &[u8],
    block: transaction::BlockHash,
    tx_count: usize,
) -> IResult<&[u8], Vec<transaction::Metadata>> {
    nom::multi::count(|i| parse_transaction(i, block), tx_count)(input)
}

fn parse_transaction(
    input: &[u8],
    block: transaction::BlockHash,
) -> IResult<&[u8], transaction::Metadata> {
    // Save original input so we can hash everything later
    let orig_input = input;

    let (input, version) = le_u32(input)?;
    let (input, input_count) = take_varint(input)?;

    // Need to deal with the optional witness flag in newer protocols versions if it's there.
    let witnesses_enabled = input_count == 0;
    let (input, input_count) =
        match cond(witnesses_enabled, tuple((take(1u8), take_varint)))(input)? {
            (_i, None) => (input, input_count),
            (i, Some((_, s))) => (i, s),
        };

    let (input, (tx_inputs, tx_outputs)) = take_tx_inputs_and_outputs(input, input_count)?;

    let input = match cond(witnesses_enabled, |x| skip_witnesses(x, input_count))(input)? {
        (_i, None) => input,
        (i, Some(_)) => i,
    };

    // Skip the locktime field
    let (input, _) = le_u32(input)?;

    let size = input.as_ptr() as usize - orig_input.as_ptr() as usize;
    let id = transaction::hash_twice(&orig_input[..size]);
    let size = size as u32;

    // TODO: correctly calculate blockheight
    let result = transaction::Metadata {
        id: id.into(),
        version,
        block,
        blockheight: u32::MAX,
        size,
    };

    Ok((input, result))
}

fn take_tx_inputs_and_outputs(
    input: &[u8],
    input_count: usize,
) -> IResult<&[u8], (Vec<transaction::Input>, Vec<transaction::Output>)> {
    let (input, tx_inputs) = nom::multi::count(|x| take_tx_input(x), input_count)(input)?;
    let (input, output_count) = take_varint(input)?;

    let (input, tx_output_values) =
        nom::multi::count(|x| take_tx_output_value(x), output_count)(input)?;

    let tx_outputs = tx_output_values
        .iter()
        .scan(0, |index, &x| {
            let cur_index = *index;
            *index += 1;
            Some(transaction::Output {
                index: cur_index,
                value: x,
            })
        })
        .collect();

    Ok((input, (tx_inputs, tx_outputs)))
}

fn take_tx_input(input: &[u8]) -> IResult<&[u8], transaction::Input> {
    let (input, (source_tx, source_index)) = tuple((take_32_bytes_as_hash, le_u32))(input)?;

    // Skip script and sequence number
    let (input, sig_len) = take_varint(input)?;
    let amt_to_skip = sig_len + 4;
    let (input, _) = take(amt_to_skip)(input)?;

    let result = transaction::Input {
        source_tx: source_tx.into(),
        source_index,
        value: None,
    };
    Ok((input, result))
}

fn take_tx_output_value(input: &[u8]) -> IResult<&[u8], transaction::Value> {
    let (input, value) = le_u64(input)?;

    // Skip script
    let (input, sig_len) = take_varint(input)?;
    let (input, _) = take(sig_len)(input)?;

    Ok((input, value))
}

fn skip_single_witness(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, len) = take_varint(input)?;
    let (input, _) = take(len)(input)?;
    Ok((input, ()))
}

fn skip_witnesses(input: &[u8], input_count: usize) -> IResult<&[u8], ()> {
    let (input, _) = nom::multi::count(skip_single_witness, input_count)(input)?;
    Ok((input, ()))
}
