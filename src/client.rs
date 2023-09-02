use anyhow::Context;
use tokio::net::TcpStream;
use tokio::sync::broadcast;

use crate::net;

pub async fn run(destination: String, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let stream = TcpStream::connect(format!("{destination}:{port}")).await?;
    let (stream, sink) = stream.into_split();

    let (sender, proxy) = broadcast::channel(1);
    let input_handle = net::stdin(sender);

    tokio::select! {
        r = input_handle => {
            tracing::debug!("input end");
            r?
        }
        r = net::tx(sink, proxy) => {
            tracing::debug!("tx end");
            r
        }
        r = net::rx(stream) => {
            tracing::debug!("rx end");
            r
        }
    }
}
