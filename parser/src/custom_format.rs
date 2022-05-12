use crate::{
    output_writer::OutputWriter,
    transaction::{Block, InputOutputPair, Transaction},
};
use bincode::serialize_into;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufWriter;

const TRANSACTION_DBFILE: &'static str = "transactions.customdb";
const BLOCK_DBFILE: &'static str = "blocks.customdb";
const IOPAIR_DBFILE: &'static str = "iopair.customdb";

pub struct CustomWriter {
    tx_writer: BufWriter<std::fs::File>,
    block_writer: BufWriter<std::fs::File>,
    iopair_writer: BufWriter<std::fs::File>,
}

impl CustomWriter {
    pub fn new() -> CustomWriter {
        CustomWriter {
            tx_writer: BufWriter::new(File::create(TRANSACTION_DBFILE).unwrap()),
            block_writer: BufWriter::new(File::create(BLOCK_DBFILE).unwrap()),
            iopair_writer: BufWriter::new(File::create(IOPAIR_DBFILE).unwrap()),
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

fn read_custom_format<T: DeserializeOwned>(custom_db_file: &str) -> Vec<T> {
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

pub fn read_custom_formats() -> (Vec<Transaction>, Vec<Block>, Vec<InputOutputPair>) {
    let txs: Vec<Transaction> = read_custom_format(TRANSACTION_DBFILE);
    let blocks: Vec<Block> = read_custom_format(BLOCK_DBFILE);
    let iopairs: Vec<InputOutputPair> = read_custom_format(IOPAIR_DBFILE);

    (txs, blocks, iopairs)
}
