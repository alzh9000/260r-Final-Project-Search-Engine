use clap::Parser;
use parser::rpc_service::{SearchClient, DEFAULT_PORT};
use parser::transaction::TxHash;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::{client, context, tokio_serde::formats::Bincode};

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

    let results = async move {
        tokio::join! {
            clients[0].get_transactions(context::current(), vec![TxHash::new_from_str("4a9b10d5769616db54bedec98cb762ac75e26642ae4750f88144c6a9bbb70000")]),
            clients[1].get_transactions(context::current(), vec![TxHash::new_from_str("4a9b10d5769616db54bedec98cb762ac75e26642ae4750f88144c6a9bbb70000")]),
        }
    }
    .await;

    println!("{:?}", results);

    Ok(())
}
