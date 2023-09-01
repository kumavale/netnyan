use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345").await?;

    let (mut socket, _) = listener.accept().await?;
    loop {
        let mut buf = vec![];
        socket.read_buf(&mut buf).await?;
        socket.write_all(&buf).await?;
    }
}
