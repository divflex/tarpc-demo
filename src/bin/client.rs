use anyhow::Ok;
use ta::add::AddClient;
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let to_add_server =
        tarpc::serde_transport::tcp::connect("localhost:9010", Json::default).await?;
    let add_client = AddClient::new(client::Config::default(), to_add_server).spawn();

    let ctx = context::current();
    for i in 1..=5 {
        println!("{:?}", add_client.add(ctx, i, 1).await?);
    }

    Ok(())
}
