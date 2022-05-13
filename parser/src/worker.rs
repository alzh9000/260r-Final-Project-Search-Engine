use parser::custom_format::{
    read_custom_format, CustomWriter, BLOCKS_DBFILE_SORTED, IOPAIRS_DBFILE_SORTED_DEST,
    IOPAIRS_DBFILE_SORTED_SRC, TRANSACTIONS_DBFILE_SORTED,
};
use parser::transaction::{Block, InputOutputPair, Transaction};

fn main() {}

fn load_data_sorted() -> (
    Vec<Transaction>,
    Vec<Block>,
    Vec<InputOutputPair>,
    Vec<InputOutputPair>,
) {
    let txs: Vec<Transaction> = read_custom_format(TRANSACTIONS_DBFILE_SORTED);
    let blocks: Vec<Block> = read_custom_format(BLOCKS_DBFILE_SORTED);
    let iopairs_sorted_src: Vec<InputOutputPair> = read_custom_format(IOPAIRS_DBFILE_SORTED_SRC);
    let iopairs_sorted_dest: Vec<InputOutputPair> = read_custom_format(IOPAIRS_DBFILE_SORTED_DEST);

    (txs, blocks, iopairs_sorted_src, iopairs_sorted_dest)
}
