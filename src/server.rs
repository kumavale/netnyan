use anyhow::Context;
use std::io::{stdout, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener,
};
use tokio::sync::watch;

pub async fn run(port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    let (stdin, proxy) = watch::channel(String::new());

    std::thread::spawn(move || loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        stdin.send(buf).unwrap();
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        tracing::info!("connect: {:?}", addr);

        let proxy = proxy.clone();
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

async fn tx(mut sink: OwnedWriteHalf, mut proxy: watch::Receiver<String>) -> anyhow::Result<()> {
    while proxy.changed().await.is_ok() {
        let buf = proxy.borrow().clone();
        sink.write_all(buf.as_bytes()).await?;
    }
    Ok(())
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
