use parser::transaction::{Input, InputOutputPair, Output, TxHash};
use rusqlite::params;

pub struct SQLiteTestDriver<'a> {
    random_tx_loader: rusqlite::Statement<'a>,

    children_querier: rusqlite::Statement<'a>,
    parents_querier: rusqlite::Statement<'a>,
}

impl<'a, 'b: 'a> SQLiteTestDriver<'a> {
    pub fn new(conn: &'b rusqlite::Connection) -> SQLiteTestDriver<'a> {
        SQLiteTestDriver {
            random_tx_loader: conn
                .prepare("SELECT id FROM transactions ORDER BY RANDOM() LIMIT ?1")
                .unwrap(),

            children_querier: conn
                .prepare("SELECT * FROM input_output_pairs WHERE input_output_pairs.src_tx = ?1")
                .unwrap(),

            parents_querier: conn
                .prepare("SELECT * FROM input_output_pairs WHERE input_output_pairs.dest_tx = ?1")
                .unwrap(),
        }
    }

    pub fn load_random_tx_hashes(&mut self, n: u32) -> Vec<TxHash> {
        let results = self
            .random_tx_loader
            .query(params![n])
            .unwrap()
            .mapped(|row| Ok(TxHash::new(row.get(0).unwrap())));

        let mut result = Vec::new();
        for r in results {
            result.push(r.unwrap())
        }
        result
    }

    fn query_children(&mut self, tx: TxHash) -> Vec<InputOutputPair> {
        let children = self
            .children_querier
            .query_map(params![tx], |row| {
                let dest: Option<Input> = match (row.get(3), row.get(4)) {
                    (Ok(dt), Ok(di)) => Some(Input {
                        dest_tx: dt,
                        dest_index: di,
                    }),
                    _ => None,
                };

                Ok(InputOutputPair {
                    source: Output {
                        src_tx: row.get(0).unwrap(),
                        src_index: row.get(1).unwrap(),
                        value: row.get(2).unwrap(),
                    },
                    dest,
                })
            })
            .unwrap();

        let mut result = Vec::new();
        for c in children {
            result.push(c.unwrap())
        }
        result
    }

    fn query_parents(&mut self, tx: TxHash) -> Vec<InputOutputPair> {
        let parents = self
            .parents_querier
            .query_map(params![tx], |row| {
                let dest: Option<Input> = match (row.get(3), row.get(4)) {
                    (Ok(dt), Ok(di)) => Some(Input {
                        dest_tx: dt,
                        dest_index: di,
                    }),
                    _ => None,
                };

                Ok(InputOutputPair {
                    source: Output {
                        src_tx: row.get(0).unwrap(),
                        src_index: row.get(1).unwrap(),
                        value: row.get(2).unwrap(),
                    },
                    dest,
                })
            })
            .unwrap();

        let mut result = Vec::new();
        for c in parents {
            result.push(c.unwrap())
        }
        result
    }
}

fn main() {
    let sqlite_connection = rusqlite::Connection::open("sqlite-small.db").unwrap();
    let mut driver = SQLiteTestDriver::new(&sqlite_connection);

    // We load transaction data so that we can make real queries.
    println!("loading random transactions to make real queries...");
    let hashes = driver.load_random_tx_hashes(2);
    println!("data loaded... ({} tx hashes)", hashes.len());

    for h in hashes {
        let results = driver.query_children(h);
        println!("children of {:?}: {:#?}", h, results);
        let results = driver.query_parents(h);
        println!("parents of {:?}: {:#?}", h, results);
    }
}
