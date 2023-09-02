use anyhow::{anyhow, Context};
use std::io::{stdout, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::broadcast;

pub async fn run(destination: String, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let stream = TcpStream::connect(format!("{destination}:{port}")).await?;
    let (stream, sink) = stream.into_split();

    let (stdin, proxy) = broadcast::channel(1);
    let input_handle = tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
        loop {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            stdin.send(buf).map_err(|e| anyhow!(e))?;
        }
    });

    tokio::select! {
        _ = input_handle => tracing::debug!("input end"),
        _ = tx(sink, proxy) => tracing::debug!("tx end"),
        _ = rx(stream) => tracing::debug!("rx end"),
    }
    Ok(())
}

async fn tx(
    mut sink: OwnedWriteHalf,
    mut proxy: broadcast::Receiver<String>,
) -> anyhow::Result<()> {
    loop {
        let buf = proxy.recv().await?;
        sink.write_all(buf.as_bytes()).await?;
    }
}

async fn rx(mut stream: OwnedReadHalf) -> anyhow::Result<()> {
    loop {
        let mut buf = vec![];
        match stream.read_buf(&mut buf).await {
            Ok(0) => return Ok(()),
            Ok(_) => {
                print!("{}", String::from_utf8_lossy(&buf));
                stdout().flush()?;
            }
            Err(e) => anyhow::bail!(e),
        }
    }
}
