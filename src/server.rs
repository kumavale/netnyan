use anyhow::Context;
use std::io::{stdout, Write};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub async fn run(port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            loop {
                let mut buf = vec![];
                socket.read_buf(&mut buf).await?;
                print!("{}", String::from_utf8_lossy(&buf));
                stdout().flush()?;
            }
            #[allow(unreachable_code)]
            Result::<(), anyhow::Error>::Ok(())
        });
    }
}
