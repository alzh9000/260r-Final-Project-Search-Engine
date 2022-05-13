use crate::{
    output_writer::OutputWriter,
    transaction::{Block, InputOutputPair, Transaction},
};
use bincode::serialize_into;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufWriter;

pub const TRANSACTIONS_DBFILE_UNSORTED: &'static str = "transactions.customdb";
pub const BLOCKS_DBFILE_UNSORTED: &'static str = "blocks.customdb";
pub const IOPAIRS_DBFILE_UNSORTED: &'static str = "iopairs.customdb";

pub const TRANSACTIONS_DBFILE_SORTED: &'static str = "sorted-transactions.customdb";
pub const BLOCKS_DBFILE_SORTED: &'static str = "sorted-blocks.customdb";
pub const IOPAIRS_DBFILE_SORTED_SRC: &'static str = "sorted-src-iopairs.customdb";
pub const IOPAIRS_DBFILE_SORTED_DEST: &'static str = "sorted-dest-iopairs.customdb";

pub struct CustomWriter {
    tx_writer: BufWriter<std::fs::File>,
    block_writer: BufWriter<std::fs::File>,
    iopair_writer: BufWriter<std::fs::File>,
}

impl CustomWriter {
    pub fn new() -> CustomWriter {
        CustomWriter::new_with_files(
            TRANSACTIONS_DBFILE_UNSORTED,
            BLOCKS_DBFILE_UNSORTED,
            IOPAIRS_DBFILE_UNSORTED,
        )
    }

    fn new_with_files(tx_dbfile: &str, blocks_dbfile: &str, iopairs_dbfile: &str) -> CustomWriter {
        CustomWriter {
            tx_writer: BufWriter::new(File::create(tx_dbfile).unwrap()),
            block_writer: BufWriter::new(File::create(blocks_dbfile).unwrap()),
            iopair_writer: BufWriter::new(File::create(iopairs_dbfile).unwrap()),
        }
    }
}

impl OutputWriter for CustomWriter {
    fn insert_tx(&mut self, tx: Transaction) {
        serialize_into(&mut self.tx_writer, &tx).unwrap();
    }

    fn insert_block(&mut self, b: Block) {
        serialize_into(&mut self.block_writer, &b).unwrap();
    }

    fn insert_iopair(&mut self, iopair: InputOutputPair) {
        serialize_into(&mut self.iopair_writer, &iopair).unwrap();
    }
}

pub fn read_custom_format<T: DeserializeOwned>(custom_db_file: &str) -> Vec<T> {
    let data = std::fs::read(custom_db_file).unwrap();
    let mut cursor = &data.as_slice()[..];
    let mut vec: Vec<T> = Vec::new();

    loop {
        match bincode::deserialize_from(&mut cursor) {
            Err(e) => match *e {
                bincode::ErrorKind::Io(e) => match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => break,
                    _ => panic!("{}", e),
                },
                f => panic!("{}", f),
            },
            Ok(t) => {
                vec.push(t);
            }
        };
    }

    vec
}

pub fn read_custom_formats(
    tx_dbfile: &str,
    blocks_dbfile: &str,
    iopairs_dbfile: &str,
) -> (Vec<Transaction>, Vec<Block>, Vec<InputOutputPair>) {
    let txs: Vec<Transaction> = read_custom_format(tx_dbfile);
    let blocks: Vec<Block> = read_custom_format(blocks_dbfile);
    let iopairs: Vec<InputOutputPair> = read_custom_format(iopairs_dbfile);

    (txs, blocks, iopairs)
}

pub fn sort_data() {
    let (mut txs, mut blocks, mut iopairs) = read_custom_formats(
        TRANSACTIONS_DBFILE_UNSORTED,
        BLOCKS_DBFILE_UNSORTED,
        IOPAIRS_DBFILE_UNSORTED,
    );

    let mut txs_out = BufWriter::new(File::create(TRANSACTIONS_DBFILE_SORTED).unwrap());
    let mut blocks_out = BufWriter::new(File::create(BLOCKS_DBFILE_SORTED).unwrap());
    let mut iopairs_by_src_out = BufWriter::new(File::create(IOPAIRS_DBFILE_SORTED_SRC).unwrap());
    let mut iopairs_by_dest_out = BufWriter::new(File::create(IOPAIRS_DBFILE_SORTED_DEST).unwrap());

    // We use unstable sorts because they are in-place and faster than stable sorts in rust. We
    // also use up vectors explicitly (with into_iter) to minimize memory usage, especially when we are sorting
    // larger data.

    txs.sort_unstable_by_key(|k| k.block);
    println!("Sorted transactions");
    for t in txs.into_iter() {
        serialize_into(&mut txs_out, &t).unwrap();
    }
    println!("Wrote sorted transactions");

    blocks.sort_unstable_by_key(|k| k.id);
    println!("Sorted blocks");
    for b in blocks.into_iter() {
        serialize_into(&mut blocks_out, &b).unwrap();
    }
    println!("Wrote sorted blocks");

    iopairs.sort_unstable_by_key(|k| k.source.src_tx);
    println!("Sorted iopairs by source tx");
    for p in iopairs.iter() {
        serialize_into(&mut iopairs_by_src_out, &p).unwrap();
    }
    println!("Wrote iopairs sorted by source tx");

    iopairs.retain(|x| match x.dest {
        Some(_) => true,
        None => false,
    });
    println!("Filtered out iopairs without dest tx");

    iopairs.sort_unstable_by_key(|k| k.dest.unwrap().dest_tx);
    println!("Sorted iopairs by dest tx");

    for p in iopairs.into_iter() {
        serialize_into(&mut iopairs_by_dest_out, &p).unwrap();
    }
    println!("Wrote iopairs sorted by dest tx");
}
