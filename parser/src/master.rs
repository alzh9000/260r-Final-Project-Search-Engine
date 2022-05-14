use clap::Parser;
use itertools::Itertools;
use parser::custom_format::{load_data_sorted, load_tx_ids_sorted};
use parser::rpc_service::{SearchClient, DEFAULT_PORT};
use parser::transaction::{BlockHash, InputOutputPair, TxHash};
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
        0 => vec![DEFAULT_PORT, args.client.len().try_into().unwrap()],
        _ => args.port,
    };

    // In the master, we load some data so that we can make real queries.
    println!("loading data...");
    let txs = load_tx_ids_sorted();
    println!("data loaded... ({} tx hashes)", txs.len());

    let mut clients: Vec<SearchClient> = Vec::new();

    for (i, c) in args.client.iter().enumerate() {
        println!(
            "Using client {} with IP address {:?}. Trying to connect... (A hang here means the client is unreachable.)",
            i, c
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

    let mut rng = rand::thread_rng();
    let now = Instant::now();
    for i in 0..2 {
        let hash = vec![*txs.choose(&mut rng).unwrap()];
        let results = get_children_of_txs(&clients, &hash).await;
        println!("children of {:?}: {:#?}", hash[0], results);
    }

    let new_now = Instant::now();
    println!("{:?}, {:?}", now, new_now.duration_since(now));

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
