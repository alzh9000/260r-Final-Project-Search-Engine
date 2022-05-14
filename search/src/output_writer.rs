use crate::transaction::{Block, InputOutputPair, Transaction};

pub trait OutputWriter {
    fn insert_tx(&mut self, tx: Transaction);
    fn insert_block(&mut self, b: Block);
    fn insert_iopair(&mut self, iopair: InputOutputPair);
}
