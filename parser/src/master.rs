use clap::Parser;
use parser::rpc_service::SearchClient;
use parser::rpc_service::PORT;
use parser::transaction::TxHash;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::{client, context, tokio_serde::formats::Bincode};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long)]
    client: Vec<Ipv4Addr>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.client.len() < 1 {
        panic!("Need at least one client!")
    }

    let mut clients: Vec<SearchClient> = Vec::new();

    for (i, c) in args.client.iter().enumerate() {
        println!(
            "Using client {} with IP address {:?}. Trying to connect... (A hang here means the client is unreachable.)",
            i, c
        );

        let transport =
            tarpc::serde_transport::tcp::connect((IpAddr::V4(*c), PORT), Bincode::default);

        let client = SearchClient::new(client::Config::default(), transport.await?).spawn();
        clients.push(client);

        println!("Connected to client {} with IP address {:?}.", i, c);
    }

    println!("Master client spawned!");

    let results = async move {
        tokio::select! {
            result1 = clients[0].get_transactions(context::current(), vec![TxHash::new_from_str("4a9b10d5769616db54bedec98cb762ac75e26642ae4750f88144c6a9bbb70000")]) => { result1 },
        }
    }
    .await;

    println!("{:?}", results);

    Ok(())
}
