use crate::output_writer::OutputWriter;
use crate::transaction::{Block, InputOutputPair, Transaction};
use rusqlite::params;
use rusqlite::Connection;

pub struct SQLiteDriver {
    conn: Connection,
}

impl SQLiteDriver {
    pub fn new() -> SQLiteDriver {
        let connection = rusqlite::Connection::open("btc-test.db").unwrap();

        connection
            .pragma_update(None, "journal_mode", "memory")
            .unwrap();

        connection
            .pragma_update(None, "synchronous", "off")
            .unwrap();

        connection
            .execute(
                "
        CREATE TABLE transactions (
            id                  BLOB NOT NULL,
            version             UNSIGNED INT4 NOT NULL,
            block               BLOB NOT NULL,
            block_height        UNSIGNED INT4 NOT NULL,
            size                UNSIGNED INT4 NOT NULL
        );",
                [],
            )
            .unwrap();

        connection
            .execute(
                "
        CREATE TABLE blocks (
            block_hash          BLOB NOT NULL,
            version             UNSIGNED INT4 NOT NULL,
            prev_block_id       BLOB NOT NULL,
            merkle_root         BLOB NOT NULL,
            unix_time           UNSIGNED INT4 NOT NULL,
            tx_count            UNSIGNED INT4 NOT NULL,
            height              UNSIGNED INT4 NOT NULL
        );",
                [],
            )
            .unwrap();

        connection
            .execute(
                "
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

        SQLiteDriver { conn: connection }
    }
}

impl OutputWriter for SQLiteDriver {
    fn insert_tx(&self, tx: Transaction) {
        self.conn
            .execute(
                "INSERT INTO transactions VALUES (?1, ?2, ?3, ?4, ?5);",
                params![tx.id, tx.version, tx.block, tx.block_height, tx.size],
            )
            .unwrap();
    }

    fn insert_block(&self, b: Block) {
        self.conn
            .execute(
                "INSERT INTO blocks VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
                params![
                    b.id,
                    b.version,
                    b.prev_block_id,
                    b.merkle_root,
                    b.unix_time,
                    b.tx_count,
                    b.height
                ],
            )
            .unwrap();
    }

    fn insert_iopair(&self, iopair: InputOutputPair) {
        let dest_tx = match iopair.dest {
            None => None,
            Some(d) => Some(d.dest_tx),
        };
        let dest_index = match iopair.dest {
            None => None,
            Some(d) => Some(d.dest_index),
        };

        self.conn
            .execute(
                "INSERT INTO input_output_pairs VALUES (?1, ?2, ?3, ?4, ?5);",
                params![
                    iopair.source.src_tx,
                    iopair.source.src_index,
                    iopair.source.value,
                    dest_tx,
                    dest_index,
                ],
            )
            .unwrap();
    }
}
