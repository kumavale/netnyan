use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

pub fn stdin(sender: broadcast::Sender<Vec<u8>>) -> JoinHandle<anyhow::Result<()>> {
    tokio::task::spawn_blocking(move || loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        sender.send(buf.into_bytes())?;
    })
}

pub async fn tx(
    mut sink: OwnedWriteHalf,
    mut proxy: broadcast::Receiver<Vec<u8>>,
) -> anyhow::Result<()> {
    loop {
        let buf = proxy.recv().await?;
        sink.write_all(&buf).await?;
    }
}

pub async fn rx(mut stream: OwnedReadHalf) -> anyhow::Result<()> {
    loop {
        let mut buf = vec![];
        match stream.read_buf(&mut buf).await {
            Ok(0) => return Ok(()),
            Ok(_) => std::io::stdout().lock().write_all(&buf)?,
            Err(e) => anyhow::bail!(e),
        }
    }
}
