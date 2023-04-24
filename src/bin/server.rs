use anyhow::Ok;
use futures::{future, prelude::*};
use ta::add::Add as AddService;
use tarpc::{
    context,
    server::{incoming::Incoming, BaseChannel},
    tokio_serde::formats::Json,
};

#[derive(Clone)]
struct AddServer;

#[tarpc::server]
impl AddService for AddServer {
    async fn add(self, _: context::Context, x: i32, y: i32) -> i32 {
        x + y
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let add_listener = tarpc::serde_transport::tcp::listen("localhost:9010", Json::default)
        .await?
        .filter_map(|r| future::ready(r.ok()));
    let add_server = add_listener
        .map(BaseChannel::with_defaults)
        .execute(AddServer.serve());
    let j = tokio::spawn(add_server);

    j.await?;

    Ok(())
}
