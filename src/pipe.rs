use anyhow::anyhow;
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::{
    broadcast::{self, error::RecvError},
    oneshot,
};
use tokio::task::JoinHandle;

pub struct ExitSignal;

pub fn stdin(sender: broadcast::Sender<Vec<u8>>) -> JoinHandle<anyhow::Result<()>> {
    tokio::task::spawn_blocking(move || loop {
        let mut buf = vec![];
        let mut chunk = std::io::stdin().lock().take(65535);
        match chunk.read_to_end(&mut buf)? {
            0 => return Ok(()),
            _ => sender.send(buf)?,
        };
    })
}

pub async fn tx(
    mut sink: OwnedWriteHalf,
    mut proxy: broadcast::Receiver<Vec<u8>>,
    exit_sig_receiver: oneshot::Receiver<ExitSignal>,
) -> anyhow::Result<()> {
    loop {
        let buf = match proxy.recv().await {
            Ok(buf) => buf,
            Err(RecvError::Closed) => break,
            Err(e) => return Err(anyhow!(e)),
        };
        sink.write_all(&buf).await?;
    }
    // Wait until `rx()` is received all data
    exit_sig_receiver.await?;
    Ok(())
}

pub async fn rx(
    mut stream: OwnedReadHalf,
    exit_sig_sender: oneshot::Sender<ExitSignal>,
) -> anyhow::Result<()> {
    loop {
        let mut buf = vec![];
        match stream.read_buf(&mut buf).await {
            Ok(0) => {
                exit_sig_sender
                    .send(ExitSignal)
                    .map_err(|_| anyhow!("failed to send exit_signal"))?;
                return Ok(());
            }
            Ok(_) => std::io::stdout().lock().write_all(&buf)?,
            Err(e) => anyhow::bail!(e),
        }
    }
}
