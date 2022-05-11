use crate::transaction::{Block, InputOutputPair, Transaction};
use rusqlite::params;
use rusqlite::Connection;

pub fn initialize() -> Connection {
    let connection = rusqlite::Connection::open("btc-test.db").unwrap();

    connection
        .execute(
            "
        CREATE TABLE transactions (
            id                  BLOB NOT NULL,
            version             UNSIGNED INT4 NOT NULL,
            block               BLOB NOT NULL,
            block_height        UNSIGNED INT4 NOT NULL,
            size                UNSIGNED INT4 NOT NULL
        );

        CREATE TABLE blocks (
            block_hash          BLOB NOT NULL,
            version             UNSIGNED INT4 NOT NULL,
            prev_block_id       BLOB NOT NULL,
            merkle_root         BLOB NOT NULL,
            unix_time           UNSIGNED INT4 NOT NULL,
            tx_count            UNSIGNED INT4 NOT NULL,
            height              UNSIGNED INT4 NOT NULL
        );

        CREATE TABLE input_output_pairs (
            src_tx              BLOB NOT NULL,
            src_index           UNSIGNED INT4 NOT NULL,
            value               UNSIGNED INT8 NOT NULL,
            dest_tx             BLOB,
            dest_index          INT4
        );",
            [],
        )
        .unwrap();

    connection
}

pub fn insert_tx(conn: &Connection, tx: Transaction) {}

pub fn insert_block(conn: &Connection, b: Block) {}

pub fn insert_iopair(conn: &Connection, iopair: InputOutputPair) {}
