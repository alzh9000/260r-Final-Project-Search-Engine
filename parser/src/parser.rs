use crate::{
    output_writer::OutputWriter,
    transaction::{self, Block, Input, InputOutputPair, Output, Transaction, TxHash},
};
use nom::{
    bytes::complete::{tag, take},
    combinator::cond,
    number::complete::{le_u16, le_u32, le_u64},
    sequence::{preceded, tuple},
    IResult, ToUsize,
};
use sha2::Digest;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash)]
struct OutputHashAndIndex {
    tx: TxHash,
    index: u32,
}

pub struct Parser {
    // The key is the expected src transaction hash and index corresponding to the input.
    unmatched_inputs: HashMap<OutputHashAndIndex, transaction::Input>,
    // The key is the source tx and index of the output.
    unmatched_outputs: HashMap<OutputHashAndIndex, transaction::Output>,

    // The drainer's relevant function is called on an item whenever it is successfully and fully
    // parsed.
    drainer: Box<dyn OutputWriter>,

    blocks_parsed: u64,
}

impl Parser {
    pub fn new(drainer: Box<dyn OutputWriter>) -> Parser {
        Parser {
            unmatched_inputs: HashMap::new(),
            unmatched_outputs: HashMap::new(),

            drainer,

            blocks_parsed: 0,
        }
    }

    pub fn parse_file(&mut self, path: &Path) {
        let file = std::fs::read(path).unwrap();
        let file = file.as_slice();

        let mut input = file;
        loop {
            input = match take_raw_block_size(input) {
                Err(_e) => return,
                Ok((_i, 0)) => return,
                Ok((i, _s)) => i,
            };

            let block: Block;
            let txs: Vec<Transaction>;

            (input, block) = self.parse_block_header_and_tx_count(input).unwrap();
            self.drainer.insert_block(block);
            (input, txs) = self.parse_transactions(input, &block).unwrap();
            for t in txs.into_iter() {
                self.drainer.insert_tx(t);
            }

            self.blocks_parsed += 1;
            if self.blocks_parsed % 500 == 0 {
                println!("Blocks parsed: {}", self.blocks_parsed);
            }
        }
    }

    pub fn parse(&mut self) {
        let mut files: Vec<String> = vec![];

        for i in 0..1 {
            files.push(format!(
                "/Volumes/SavvyT7Red/BitcoinCore/blocks/blk{:05}.dat",
                i
            ));
        }

        for (i, f) in files.iter().enumerate() {
            println!("Parsing file {} of {}...: {}", i, files.len(), &f);
            self.parse_file(Path::new(f));
        }

        self.finalize();
    }

    // Note that height is not correct when this function returns.
    fn parse_block_header_and_tx_count<'a, 'b>(
        &'a mut self,
        input: &'b [u8],
    ) -> IResult<&'b [u8], transaction::Block> {
        let (input, header) = take(80u8)(input)?;

        // hash entire header to get the block ID
        let id = hash_twice(header);

        let mut parser = tuple((le_u32, take_32_bytes_as_hash, take_32_bytes_as_hash, le_u32));
        let (_, (version, prev_id, merkle_root, unix_time)) = parser(header)?;
        let (input, tx_count) = take_varint_fixed(input)?;

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

    fn parse_transaction<'a, 'b>(
        &mut self,
        input: &'a [u8],
        block: &'b transaction::Block,
    ) -> IResult<&'a [u8], transaction::Transaction> {
        // Save original input so we can hash everything later
        let orig_input = input;

        let (input, version) = le_u32(input)?;
        let (input, input_count) = take_varint_fixed(input)?;

        // Need to deal with the optional witness flag in newer protocols versions if it's there.
        let witnesses_enabled = input_count == 0;
        let (input, input_count) =
            match cond(witnesses_enabled, tuple((take(1u8), take_varint_fixed)))(input)? {
                (_i, None) => (input, input_count),
                (i, Some((_, s))) => (i, s),
            };

        // Take the raw data from the inputs and outputs
        let (input, tx_input_sources) =
            nom::multi::count(take_tx_input, input_count.to_usize())(input)?;
        let (input, output_count) = take_varint_fixed(input)?;
        let (input, tx_output_values) =
            nom::multi::count(|x| take_tx_output_value(x), output_count.to_usize())(input)?;

        // Skip witnesses if we need to
        let input = match cond(witnesses_enabled, |x| {
            skip_witnesses(x, input_count.to_usize())
        })(input)?
        {
            (_i, None) => input,
            (i, Some(_)) => i,
        };

        // Skip the locktime field
        let (input, _) = le_u32(input)?;

        // Compute size and hash
        let size = input.as_ptr() as usize - orig_input.as_ptr() as usize;
        let id = hash_twice(&orig_input[..size]);
        let size = size as u32;

        // Compute resulting transaction
        let result = transaction::Transaction {
            id: id.into(),
            version,
            block: block.id,
            block_height: block.height,
            size,
        };

        // For each output and input, register what we parsed
        for (i, v) in tx_output_values.into_iter().enumerate() {
            self.register_output(Output {
                src_tx: id.into(),
                src_index: i.try_into().unwrap(),
                value: v,
            })
        }

        for (i, v) in tx_input_sources.into_iter().enumerate() {
            self.register_input(
                Input {
                    dest_tx: id.into(),
                    dest_index: i.try_into().unwrap(),
                },
                v.tx,
                v.index,
            )
        }

        Ok((input, result))
    }

    fn parse_transactions<'a, 'b>(
        &mut self,
        input: &'a [u8],
        block: &'b transaction::Block,
    ) -> IResult<&'a [u8], Vec<transaction::Transaction>> {
        nom::multi::count(
            |i| self.parse_transaction(i, block),
            block.tx_count.to_usize(),
        )(input)
    }

    fn register_input(&mut self, i: Input, expected_src_tx: TxHash, expected_src_index: u32) {
        let key = OutputHashAndIndex {
            tx: expected_src_tx,
            index: expected_src_index,
        };
        match self.unmatched_outputs.get(&key) {
            None => {
                self.unmatched_inputs.insert(key, i);
            }
            Some(o) => {
                self.drainer.insert_iopair(InputOutputPair {
                    source: *o,
                    dest: Some(i),
                });
                self.unmatched_outputs.remove(&key);
            }
        }
    }

    fn register_output(&mut self, o: Output) {
        let key = OutputHashAndIndex {
            tx: o.src_tx,
            index: o.src_index,
        };
        match self.unmatched_inputs.get(&key) {
            None => {
                self.unmatched_outputs.insert(key, o);
            }
            Some(i) => {
                self.drainer.insert_iopair(InputOutputPair {
                    source: o,
                    dest: Some(*i),
                });
                self.unmatched_inputs.remove(&key);
            }
        }
    }

    fn finalize(&mut self) {
        println!(
            "Finalizing! Writing {} tx outputs without corresponding inputs into the database",
            self.unmatched_outputs.len(),
        );

        for u in self.unmatched_outputs.values() {
            self.drainer.insert_iopair(InputOutputPair {
                source: *u,
                dest: None,
            });
        }
    }
}

fn take_raw_block_size(input: &[u8]) -> IResult<&[u8], u32> {
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

// returns a source transaction and index. The index (and tx hash) of the input itself
// will be taken care of by the calling function.
fn take_tx_input(input: &[u8]) -> IResult<&[u8], OutputHashAndIndex> {
    let (input, (src_tx, src_index)) = tuple((take_32_bytes_as_hash, le_u32))(input)?;

    // Skip script and sequence number
    let (input, sig_len) = take_varint_fixed(input)?;
    let amt_to_skip = sig_len + 4;

    let (input, _) = take(amt_to_skip)(input)?;

    Ok((
        input,
        OutputHashAndIndex {
            tx: src_tx.into(),
            index: src_index,
        },
    ))
}

fn take_tx_output_value(input: &[u8]) -> IResult<&[u8], transaction::Value> {
    let (input, value) = le_u64(input)?;

    // Skip script
    let (input, sig_len) = take_varint_fixed(input)?;
    let (input, _) = take(sig_len)(input)?;

    Ok((input, value))
}

fn skip_single_witness_stack_item(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, len) = take_varint_fixed(input)?;
    let (input, _) = take(len)(input)?;
    Ok((input, ()))
}

fn skip_single_witness(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, stack_count) = take_varint_fixed(input)?;
    let (input, _) =
        nom::multi::count(skip_single_witness_stack_item, stack_count.to_usize())(input)?;
    Ok((input, ()))
}

fn skip_witnesses(input: &[u8], input_count: usize) -> IResult<&[u8], ()> {
    let (input, _) = nom::multi::count(skip_single_witness, input_count)(input)?;
    Ok((input, ()))
}

fn take_varint_fixed(input: &[u8]) -> IResult<&[u8], u64> {
    let (input, first_byte) = take(1u8)(input)?;
    let first_byte = first_byte[0];
    if first_byte < 0xFD {
        Ok((input, first_byte.into()))
    } else if first_byte == 0xFD {
        let (input, val) = le_u16(input)?;
        Ok(((input), val.into()))
    } else if first_byte == 0xFE {
        let (input, val) = le_u32(input)?;
        Ok(((input), val.into()))
    } else {
        return le_u64(input);
    }
}

// Hash helpers

pub fn hash_once(x: &[u8]) -> transaction::Hash256 {
    sha2::Sha256::digest(x).into()
}

pub fn hash_twice(x: &[u8]) -> transaction::Hash256 {
    hash_once(&hash_once(x))
}
