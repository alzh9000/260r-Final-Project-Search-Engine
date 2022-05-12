use std::fs::File;
use std::io::BufWriter;

use bincode::serialize_into;

use crate::{
    output_writer::OutputWriter,
    transaction::{Block, InputOutputPair, Transaction},
};

pub struct CustomWriter {
    tx_writer: BufWriter<std::fs::File>,
    block_writer: BufWriter<std::fs::File>,
    iopair_writer: BufWriter<std::fs::File>,
}

impl CustomWriter {
    pub fn new() -> CustomWriter {
        CustomWriter {
            tx_writer: BufWriter::new(File::create("transactions.customdb").unwrap()),
            block_writer: BufWriter::new(File::create("blocks.customdb").unwrap()),
            iopair_writer: BufWriter::new(File::create("iopairs.customdb").unwrap()),
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
