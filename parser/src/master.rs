use clap::Parser;
use parser::rpc_service::{SearchClient, DEFAULT_PORT};
use parser::transaction::{TxHash, BlockHash};
use std::net::{IpAddr, Ipv4Addr};
use tarpc::{client, context, tokio_serde::formats::Bincode};
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

    // let hashes = &["0f1d7406160f976ab69458811a386ebe444fcc8bf9b36a7ac27641b8182f8ee1", "5141b1d6eac1f5106fa709cec4aa7ec3a7d7b962d46c48a88899d9fa1dd40131", "41649a6830cc8092b926d9f66536efc74f552a44d88cf32543ab94406f220100"];
    let hashes = &["5141b1d6eac1f5106fa709cec4aa7ec3a7d7b962d46c48a88899d9fa1dd40131", "41649a6830cc8092b926d9f66536efc74f552a44d88cf32543ab94406f220100"];
    // let block_hashes = &["00000000000d6f206bec856b367e64dbcfdbbc2b31ea087c0fb834b0d15b0000", "00000000659e1402f1cdf3d2fd94065369e5cde43d6a00b6b5edcff01db50000", "0000000000006e3d40fbf76994f48d2204382076a706ecec921abea6d10b0200"];
    // let block_hashes = &["00000000659e1402f1cdf3d2fd94065369e5cde43d6a00b6b5edcff01db50000", "0000000000006e3d40fbf76994f48d2204382076a706ecec921abea6d10b0200"];
    let now = Instant::now();
    for hash in hashes {
    let results = async {
        tokio::join! {
            // clients[0].get_transactions(context::current(), vec![TxHash::new_from_str(hash)])
            // clients[1].get_transactions(context::current(), vec![TxHash::new_from_str("4a9b10d5769616db54bedec98cb762ac75e26642ae4750f88144c6a9bbb70000")]),

            // clients[0].get_blocks(context::current(), vec![BlockHash::new_from_str(hash)])

            clients[0].transactions_by_sources(context::current(), vec![TxHash::new_from_str(hash)])
        }
        }
        .await;
    }   

    let new_now = Instant::now();
    println!("{:?}, {:?}", now, new_now.duration_since(now));
    

    Ok(())
}
