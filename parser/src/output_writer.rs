use crate::transaction::{Block, InputOutputPair, Transaction};

pub trait OutputWriter {
    fn insert_tx(&self, tx: Transaction);
    fn insert_block(&self, b: Block);
    fn insert_iopair(&self, iopair: InputOutputPair);
}
