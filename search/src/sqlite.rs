use crate::output_writer::OutputWriter;
use crate::transaction::{Block, InputOutputPair, Transaction};
use rusqlite::params;

pub struct SQLiteDriver<'a> {
    tx_inserter: rusqlite::Statement<'a>,
    block_inserter: rusqlite::Statement<'a>,
    iopair_inserter: rusqlite::Statement<'a>,
}

impl<'a, 'b: 'a> SQLiteDriver<'a> {
    pub fn new(conn: &'b rusqlite::Connection) -> SQLiteDriver<'a> {
        conn.pragma_update(None, "journal_mode", "memory").unwrap();

        conn.pragma_update(None, "synchronous", "off").unwrap();

        conn.execute(
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

        conn.execute(
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

        conn.execute(
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

        SQLiteDriver {
            tx_inserter: conn
                .prepare("INSERT INTO transactions VALUES (?1, ?2, ?3, ?4, ?5);")
                .unwrap(),
            block_inserter: conn
                .prepare("INSERT INTO blocks VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);")
                .unwrap(),
            iopair_inserter: conn
                .prepare("INSERT INTO input_output_pairs VALUES (?1, ?2, ?3, ?4, ?5);")
                .unwrap(),
        }
    }
}

impl<'a> OutputWriter for SQLiteDriver<'a> {
    fn insert_tx(&mut self, tx: Transaction) {
        self.tx_inserter
            .execute(params![
                tx.id,
                tx.version,
                tx.block,
                tx.block_height,
                tx.size
            ])
            .unwrap();
    }

    fn insert_block(&mut self, b: Block) {
        self.block_inserter
            .execute(params![
                b.id,
                b.version,
                b.prev_block_id,
                b.merkle_root,
                b.unix_time,
                b.tx_count,
                b.height
            ])
            .unwrap();
    }

    fn insert_iopair(&mut self, iopair: InputOutputPair) {
        let dest_tx = match iopair.dest {
            None => None,
            Some(d) => Some(d.dest_tx),
        };
        let dest_index = match iopair.dest {
            None => None,
            Some(d) => Some(d.dest_index),
        };

        self.iopair_inserter
            .execute(params![
                iopair.source.src_tx,
                iopair.source.src_index,
                iopair.source.value,
                dest_tx,
                dest_index,
            ])
            .unwrap();
    }
}
