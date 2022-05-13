use parser::rpc_service::SearchClient;
use std::net::{IpAddr, Ipv6Addr};
use tarpc::{client, context, tokio_serde::formats::Bincode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let transport = tarpc::serde_transport::tcp::connect(
        (IpAddr::V6(Ipv6Addr::LOCALHOST), 6969),
        Bincode::default,
    );

    let client = SearchClient::new(client::Config::default(), transport.await?).spawn();

    println!("Master client spawned!");

    let results = async move {
        tokio::select! {
            result1 = client.get_transactions(context::current(), vec![]) => { result1 },
            result2 = client.get_transactions(context::current(), vec![]) => { result2 },
        }
    }
    .await;

    println!("{:?}", results);

    Ok(())
}
