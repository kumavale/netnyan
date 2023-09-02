use anyhow::Context;
use std::io::{stdout, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

pub async fn run(destination: String, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let stream = TcpStream::connect(format!("{destination}:{port}")).await?;
    let (stream, sink) = stream.into_split();
    let (tx, rx) = tokio::join!(tx(sink), rx(stream));
    tx?;
    rx?;
    Ok(())
}

async fn tx(mut sink: OwnedWriteHalf) -> anyhow::Result<()> {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        sink.write_all(buf.as_bytes()).await?;
    }
}

async fn rx(mut stream: OwnedReadHalf) -> anyhow::Result<()> {
    loop {
        let mut buf = vec![];
        stream.read_buf(&mut buf).await?;
        print!("{}", String::from_utf8_lossy(&buf));
        stdout().flush()?;
    }
}
