use anyhow::Context;
use std::io::IsTerminal;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, oneshot};

pub async fn run(destination: String, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let stream = TcpStream::connect(format!("{destination}:{port}")).await?;
    let (stream, sink) = stream.into_split();
    let (sender, proxy) = broadcast::channel(1);

    if std::io::stdin().is_terminal() {
        use crate::net;
        tracing::debug!("from stdin");
        tokio::spawn(net::stdin(sender));
        tokio::select! {
            r = net::tx(sink, proxy) => { tracing::debug!("tx end"); r }
            r = net::rx(stream)      => { tracing::debug!("rx end"); r }
        }
    } else {
        use crate::pipe;
        tracing::debug!("from pipe");
        let (exit_sig_sender, exit_sig_receiver) = oneshot::channel::<pipe::ExitSignal>();
        tokio::spawn(pipe::stdin(sender));
        tokio::select! {
            r = pipe::tx(sink, proxy, exit_sig_receiver) => { tracing::debug!("tx end"); r }
            r = pipe::rx(stream, exit_sig_sender)        => { tracing::debug!("rx end"); r }
        }
    }
}
