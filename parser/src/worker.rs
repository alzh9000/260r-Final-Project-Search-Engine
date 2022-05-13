use parser::custom_format::{
    read_custom_format, sort_data, CustomWriter, BLOCKS_DBFILE_SORTED, IOPAIRS_DBFILE_SORTED_DEST,
    IOPAIRS_DBFILE_SORTED_SRC, TRANSACTIONS_DBFILE_SORTED,
};
use parser::transaction::{Block, InputOutputPair, Transaction};

fn main() {}

fn load_data_sorted() -> () {
    let txs: Vec<Transaction> = read_custom_format(TRANSACTIONS_DBFILE_SORTED);
}
