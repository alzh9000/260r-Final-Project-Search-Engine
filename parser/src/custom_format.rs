use crate::{
    output_writer::OutputWriter,
    transaction::{Block, InputOutputPair, Transaction},
};

pub struct CustomWriter {}

impl CustomWriter {
    pub fn new() -> CustomWriter {
        CustomWriter {}
    }
}

impl OutputWriter for CustomWriter {
    fn insert_tx(&self, tx: Transaction) {}

    fn insert_block(&self, b: Block) {}

    fn insert_iopair(&self, iopair: InputOutputPair) {}
}
