use sqlite::{self, Connection};

use crate::transaction::{Block, InputOutputPair, Transaction};

pub fn initialize() -> Connection {
    let connection = sqlite::open("btc-test.db").unwrap();

    connection.execute(
        "
        CREATE TABLE blocks (block_hash BLOB NOT NULL, version UNSIGNED INT4 NOT NULL, prev_block_id BLOB NOT NULL, merkle_root BLOB NOT NULL, unix_time UNSIGNED INT4 NOT NULL, tx_count UNSIGNED INT4 NOT NULL, height UNSIGNED INT4 NOT NULL);
        CREATE TABLE transactions (id BLOB NOT NULL, version UNSIGNED INT4 NOT NULL, block BLOB NOT NULL, block_height UNSIGNED INT4 NOT NULL, size UNSIGNED INT4 NOT NULL);
        CREATE TABLE input_output_pairs (src_tx BLOB NOT NULL, src_index UNSIGNED INT4 NOT NULL, value UNSIGNED INT8 NOT NULL, dest_tx BLOB, dest_index INT4);
        "
    ,) .unwrap();

    connection

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

pub fn insert_tx(conn: &Connection, tx: Transaction) {}

pub fn insert_block(conn: &Connection, b: Block) {}

pub fn insert_iopair(conn: &Connection, iopair: InputOutputPair) {}
