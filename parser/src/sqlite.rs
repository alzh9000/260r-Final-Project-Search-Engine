use sqlite;

use crate::transaction::{Block, InputOutputPair, Transaction};

pub fn initialize() -> () {
    let connection = sqlite::open("/Volumes/SavvyT7Red/Sqlite/btc.db").unwrap();

    connection.execute(
        "
        CREATE TABLE blocks (block_hash BLOB NOT NULL, version UNSIGNED INT4 NOT NULL, prev_block_id BLOB NOT NULL, merkle_root BLOB NOT NULL, unix_time UNSIGNED INT4 NOT NULL, tx_count UNSIGNED INT4 NOT NULL, height UNSIGNED INT4 NOT NULL);
        CREATE TABLE transactions (tx_hash BLOB, version UNSIGNED INT4, block BLOB, block_height UNSIGNED INT4, size UNSIGNED INT4);
        "
    ,) .unwrap();

    // TODO: create prepared statements for data insertion and the queries we want to run

    // connection
    // .iterate("SELECT * FROM blocks WHERE version > 1", |pairs| {
    // for &(column, value) in pairs.iter() {
    // println!("{} = {}", column, value.unwrap());
    // }
    // true
    // })
    // .unwrap();
}

pub fn insert_tx(tx: Transaction) {}

pub fn insert_block(b: Block) {}

pub fn insert_iopair(iopair: InputOutputPair) {}
