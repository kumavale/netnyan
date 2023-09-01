use anyhow::Context;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn run(destination: String, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let mut stream = TcpStream::connect(format!("{destination}:{port}")).await?;
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        stream.write_all(buf.as_bytes()).await?;
    }
}
