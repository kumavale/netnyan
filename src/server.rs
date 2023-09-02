use anyhow::Context;
use std::io::{stdout, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener,
};
use tokio::sync::broadcast;

pub async fn run(port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let (stdin, proxy) = broadcast::channel(1);

    std::thread::spawn(move || loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        stdin.send(buf).unwrap();
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::info!("connect: {:?}", addr);

        let proxy = proxy.resubscribe();
        tokio::spawn(async move {
            let (stream, sink) = socket.into_split();
            tokio::select! {
                _ = tokio::spawn(tx(sink, proxy)) => (),
                _ = tokio::spawn(rx(stream)) => (),
            }
            tracing::info!("disconnect: {:?}", addr);
        });
    }
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
            Err(e) => tracing::error!("{e}"),
        }
    }
}
