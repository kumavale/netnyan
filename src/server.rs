use anyhow::Context;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run(port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    let (mut socket, _) = listener.accept().await?;
    loop {
        let mut buf = vec![];
        socket.read_buf(&mut buf).await?;
        socket.write_all(&buf).await?;
    }
}
