use clap::Parser;
use hdrhistogram::Histogram;
use search::custom_format::load_tx_ids_sorted;
use search::rpc_service::{SearchClient, DEFAULT_PORT};
use search::transaction::{InputOutputPair, TxHash};
use rand::seq::SliceRandom;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::{client, context, tokio_serde::formats::Bincode};
use tokio;
use tokio::time::Instant;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long)]
    client: Vec<Ipv4Addr>,

    #[clap(short, long)]
    port: Vec<u16>,
}

const THROUGHPUT_NUM_ITERS: u64 = 100_000;
const LATENCY_NUM_ITERS: u64 = 100_000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.client.len() < 1 {
        panic!("Need at least one client!")
    }
    if args.port.len() != 0 && args.client.len() != args.port.len() {
        panic!(
            "Passed in {} client IP addresses and {} port numbers. They need to match (or you can pass in no ports at all, and the default of {} is used for all clients).",
            args.client.len(),
            args.port.len(),
            DEFAULT_PORT,
        );
    }
    let ports: Vec<u16> = match args.port.len() {
        0 => vec![DEFAULT_PORT; args.client.len().try_into().unwrap()],
        _ => args.port,
    };

    // In the master, we load some data so that we can make real queries.
    println!("loading data...");
    let txs = load_tx_ids_sorted();
    println!("data loaded... ({} tx hashes)", txs.len());

    let mut clients: Vec<SearchClient> = Vec::new();

    for (i, c) in args.client.iter().enumerate() {
        println!(
            "Using client {} with IP address {:?}:{}. Trying to connect... (A hang here means the client is unreachable.)",
            i, c, ports[i]
        );

        let transport =
            tarpc::serde_transport::tcp::connect((IpAddr::V4(*c), ports[i]), Bincode::default);

        let client = SearchClient::new(client::Config::default(), transport.await?).spawn();
        clients.push(client);

        println!(
            "Connected to client {} with address {:?}:{}.",
            i, c, ports[i]
        );
    }
    println!("Master clients spawned!");
    println!("");

    // Testing setup
    let mut rng = rand::thread_rng();

    // Run tests for children queries

    {
        println!("Children queries throughput test...");
        let now = Instant::now();
        for _i in 0..THROUGHPUT_NUM_ITERS {
            let hash = vec![*txs.choose(&mut rng).unwrap()];
            let _results = get_children_of_txs(&clients, &hash).await;

            // println!("children of {:?}: {:#?}", hash[0], _results);
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

        for _i in 0..LATENCY_NUM_ITERS {
            let hash = vec![*txs.choose(&mut rng).unwrap()];
            let now = Instant::now();
            let _results = get_children_of_txs(&clients, &hash).await;
            let new_now = Instant::now();

            latencies_ns
                .record(new_now.duration_since(now).as_nanos().try_into().unwrap())
                .unwrap();

            // println!("children of {:?}: {:#?}", hash[0], _results);
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
        for _i in 0..THROUGHPUT_NUM_ITERS {
            let hash = vec![*txs.choose(&mut rng).unwrap()];
            let _results = get_parents_of_txs(&clients, &hash).await;

            // println!("parents of {:?}: {:#?}", hash[0], _results);
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

        for _i in 0..LATENCY_NUM_ITERS {
            let hash = vec![*txs.choose(&mut rng).unwrap()];
            let now = Instant::now();
            let _results = get_parents_of_txs(&clients, &hash).await;
            let new_now = Instant::now();

            latencies_ns
                .record(new_now.duration_since(now).as_nanos().try_into().unwrap())
                .unwrap();

            // println!("parents of {:?}: {:#?}", hash[0], _results);
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

    Ok(())
}

async fn get_children_of_txs(clients: &Vec<SearchClient>, t: &Vec<TxHash>) -> Vec<InputOutputPair> {
    match clients.len() {
        1 => {
            match async {
                tokio::join! {
                    clients[0].transactions_by_sources(context::current(), t.to_vec())
                }
            }
            .await
            {
                (Ok(v),) => v,
                (Err(e),) => panic!("{}", e),
            }
        }
        2 => {
            match async {
                tokio::join! {
                    clients[0].transactions_by_sources(context::current(), t.to_vec()),
                    clients[1].transactions_by_sources(context::current(), t.to_vec()),
                }
            }
            .await
            {
                (x, y) => {
                    let (mut x, mut y) = (x.unwrap(), y.unwrap());
                    let mut result: Vec<InputOutputPair> = Vec::new();
                    result.append(&mut x);
                    result.append(&mut y);
                    result.sort_unstable();
                    result.dedup();
                    result
                }
            }
        },
        3 => {
            match async {
                tokio::join! {
                    clients[0].transactions_by_sources(context::current(), t.to_vec()),
                    clients[1].transactions_by_sources(context::current(), t.to_vec()),
                    clients[2].transactions_by_sources(context::current(), t.to_vec()),
                }
            }
            .await
            {
                (x, y, z) => {
                    let (mut x, mut y, mut z) = (x.unwrap(), y.unwrap(), z.unwrap());
                    let mut result: Vec<InputOutputPair> = Vec::new();
                    result.append(&mut x);
                    result.append(&mut y);
                    result.append(&mut z);
                    result.sort_unstable();
                    result.dedup();
                    result
                }
            }
        },
        _ => panic!("Because of personal issues with the Rust compiler, we currently only support the cases where there are exactly 1, 2, or 3 clients.")
    }
}

async fn get_parents_of_txs(clients: &Vec<SearchClient>, t: &Vec<TxHash>) -> Vec<InputOutputPair> {
    match clients.len() {
        1 => {
            match async {
                tokio::join! {
                    clients[0].transactions_by_destinations(context::current(), t.to_vec())
                }
            }
            .await
            {
                (Ok(v),) => v,
                (Err(e),) => panic!("{}", e),
            }
        },
        2 => {
            match async {
                tokio::join! {
                    clients[0].transactions_by_destinations(context::current(), t.to_vec()),
                    clients[1].transactions_by_destinations(context::current(), t.to_vec()),
                }
            }
            .await
            {
                (x, y) => {
                    let (mut x, mut y) = (x.unwrap(), y.unwrap());
                    let mut result: Vec<InputOutputPair> = Vec::new();
                    result.append(&mut x);
                    result.append(&mut y);
                    result.sort_unstable();
                    result.dedup();
                    result
                }
            }
        },
        3 => {
            match async {
                tokio::join! {
                    clients[0].transactions_by_destinations(context::current(), t.to_vec()),
                    clients[1].transactions_by_destinations(context::current(), t.to_vec()),
                    clients[2].transactions_by_destinations(context::current(), t.to_vec()),
                }
            }
            .await
            {
                (x, y, z) => {
                    let (mut x, mut y, mut z) = (x.unwrap(), y.unwrap(), z.unwrap());
                    let mut result: Vec<InputOutputPair> = Vec::new();
                    result.append(&mut x);
                    result.append(&mut y);
                    result.append(&mut z);
                    result.sort_unstable();
                    result.dedup();
                    result
                }
            }
        },
        _ => panic!("Because of personal issues with the Rust compiler, we currently only support the cases where there are exactly 1, 2, or 3 clients.")
    }
}

async fn get_grandchildren_of_tx(clients: &Vec<SearchClient>, t: &TxHash) -> Vec<InputOutputPair> {
    let v = vec![*t];
    let children = get_children_of_txs(clients, &v);
    let mut children: Vec<TxHash> = children
        .await
        .iter()
        .map(|x| x.dest.unwrap().dest_tx)
        .collect();
    children.sort();
    children.dedup();
    return get_children_of_txs(clients, &children).await;
}

async fn get_grandparents_of_tx(clients: &Vec<SearchClient>, t: &TxHash) -> Vec<InputOutputPair> {
    let v = vec![*t];
    let parents = get_parents_of_txs(clients, &v);
    let mut parents: Vec<TxHash> = parents
        .await
        .iter()
        .map(|x| x.dest.unwrap().dest_tx)
        .collect();
    parents.sort();
    parents.dedup();
    return get_parents_of_txs(clients, &parents).await;
}
