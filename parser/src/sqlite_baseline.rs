use hdrhistogram::Histogram;
use parser::transaction::{Input, InputOutputPair, Output, TxHash};
use rusqlite::params;
use std::cmp::max;
use tokio::time::Instant;

const THROUGHPUT_NUM_ITERS: usize = 100_000;
const LATENCY_NUM_ITERS: usize = 100_000;

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

    pub fn load_random_tx_hashes(&mut self, n: usize) -> Vec<TxHash> {
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
    let sqlite_connection = rusqlite::Connection::open("sqlite.db").unwrap();
    let mut driver = SQLiteTestDriver::new(&sqlite_connection);

    // We load transaction data so that we can make real queries.
    println!("loading random transactions to make real queries...");
    let hashes = driver.load_random_tx_hashes(max(THROUGHPUT_NUM_ITERS, LATENCY_NUM_ITERS));
    println!("data loaded... ({} tx hashes)", hashes.len());

    // Run tests for children queries

    {
        println!("Children queries throughput test...");
        let now = Instant::now();
        for i in 0..THROUGHPUT_NUM_ITERS {
            let h = hashes[i];
            let _results = driver.query_children(h);

            // println!("children of {:?}: {:#?}", h, _results);
        }
        let new_now = Instant::now();
        println!(
            "Children queries throughput test with {} iterations took: {:?}",
            THROUGHPUT_NUM_ITERS,
            new_now.duration_since(now)
        );
        println!("");
    }

    {
        println!("Children queries latency test...");
        let mut latencies_ns = Histogram::<u64>::new(3).unwrap();

        for i in 0..LATENCY_NUM_ITERS {
            let h = hashes[i];
            let now = Instant::now();
            let _results = driver.query_children(h);
            let new_now = Instant::now();

            latencies_ns
                .record(new_now.duration_since(now).as_nanos().try_into().unwrap())
                .unwrap();

            // println!("children of {:?}: {:#?}", h, _results);
        }
        println!(
            "Children queries latency test with {} iterations complete. Statistics:",
            THROUGHPUT_NUM_ITERS,
        );
        println!("Mean latency: {} ns", latencies_ns.mean());
        println!("Std deviation: {} ns", latencies_ns.stdev());
        println!("Min latency: {} ns", latencies_ns.min());
        println!("p25 latency: {} ns", latencies_ns.value_at_quantile(0.25));
        println!("p50 latency: {} ns", latencies_ns.value_at_quantile(0.50));
        println!("p75 latency: {} ns", latencies_ns.value_at_quantile(0.75));
        println!("p90 latency: {} ns", latencies_ns.value_at_quantile(0.90));
        println!("p95 latency: {} ns", latencies_ns.value_at_quantile(0.95));
        println!("p99 latency: {} ns", latencies_ns.value_at_quantile(0.99));
        println!(
            "p99.9 latency: {} ns",
            latencies_ns.value_at_quantile(0.999)
        );
        println!(
            "p99.99 latency: {} ns",
            latencies_ns.value_at_quantile(0.9999)
        );
        println!(
            "p99.999 latency: {} ns",
            latencies_ns.value_at_quantile(0.99999)
        );
        println!("Max latency: {} ns", latencies_ns.max());
        println!("");
    }

    // Run tests for parents queries

    {
        println!("parents queries throughput test...");
        let now = Instant::now();
        for i in 0..THROUGHPUT_NUM_ITERS {
            let h = hashes[i];
            let _results = driver.query_parents(h);

            // println!("parents of {:?}: {:#?}", h, _results);
        }
        let new_now = Instant::now();
        println!(
            "parents queries throughput test with {} iterations took: {:?}",
            THROUGHPUT_NUM_ITERS,
            new_now.duration_since(now)
        );
        println!("");
    }

    {
        println!("parents queries latency test...");
        let mut latencies_ns = Histogram::<u64>::new(3).unwrap();

        for i in 0..LATENCY_NUM_ITERS {
            let h = hashes[i];
            let now = Instant::now();
            let _results = driver.query_parents(h);
            let new_now = Instant::now();

            latencies_ns
                .record(new_now.duration_since(now).as_nanos().try_into().unwrap())
                .unwrap();

            // println!("parents of {:?}: {:#?}", h, _results);
        }
        println!(
            "parents queries latency test with {} iterations complete. Statistics:",
            THROUGHPUT_NUM_ITERS,
        );
        println!("Mean latency: {} ns", latencies_ns.mean());
        println!("Std deviation: {} ns", latencies_ns.stdev());
        println!("Min latency: {} ns", latencies_ns.min());
        println!("p25 latency: {} ns", latencies_ns.value_at_quantile(0.25));
        println!("p50 latency: {} ns", latencies_ns.value_at_quantile(0.50));
        println!("p75 latency: {} ns", latencies_ns.value_at_quantile(0.75));
        println!("p90 latency: {} ns", latencies_ns.value_at_quantile(0.90));
        println!("p95 latency: {} ns", latencies_ns.value_at_quantile(0.95));
        println!("p99 latency: {} ns", latencies_ns.value_at_quantile(0.99));
        println!(
            "p99.9 latency: {} ns",
            latencies_ns.value_at_quantile(0.999)
        );
        println!(
            "p99.99 latency: {} ns",
            latencies_ns.value_at_quantile(0.9999)
        );
        println!(
            "p99.999 latency: {} ns",
            latencies_ns.value_at_quantile(0.99999)
        );
        println!("Max latency: {} ns", latencies_ns.max());
        println!("");
    }
}
