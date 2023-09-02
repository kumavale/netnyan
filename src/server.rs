use anyhow::Context;
use tokio::net::TcpListener;
use tokio::sync::broadcast;

use crate::net;

pub async fn run(port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let socket = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    let (sender, proxy) = broadcast::channel(1);
    let input_handle = net::stdin(sender);
    let server_handle = listener(socket, proxy);

    tokio::select! {
        r = input_handle => {
            tracing::debug!("input end");
            r?
        }
        r = server_handle => {
            tracing::debug!("server end");
            r
        }
    }
}

async fn listener(listener: TcpListener, proxy: broadcast::Receiver<String>) -> anyhow::Result<()> {
    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::info!("connect: {:?}", addr);

        let proxy = proxy.resubscribe();
        tokio::spawn(async move {
            let (stream, sink) = socket.into_split();
            tokio::select! {
                _ = net::tx(sink, proxy) => (),
                _ = net::rx(stream) => (),
            }
            tracing::info!("disconnect: {:?}", addr);
        });
    }
}
