use anyhow::{anyhow, Context};
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

    let input_handle = tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
        loop {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            stdin.send(buf).map_err(|e| anyhow!(e))?;
        }
    });

    let server_handle = async {
        loop {
            let (socket, addr) = listener.accept().await?;
            tracing::info!("connect: {:?}", addr);

            let proxy = proxy.resubscribe();
            tokio::spawn(async move {
                let (stream, sink) = socket.into_split();
                tokio::select! {
                    _ = tx(sink, proxy) => (),
                    _ = rx(stream) => (),
                }
                tracing::info!("disconnect: {:?}", addr);
            });
        }
        #[allow(unreachable_code)]
        Result::<_, anyhow::Error>::Ok(())
    };

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
