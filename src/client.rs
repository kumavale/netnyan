use anyhow::Context;
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, error::RecvError};

use crate::net;

pub async fn run(destination: String, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.context("missing port number")?;
    let stream = TcpStream::connect(format!("{destination}:{port}")).await?;
    let (stream, sink) = stream.into_split();

    let (sender, proxy) = broadcast::channel(1);
    let input_handle = if atty::is(atty::Stream::Stdin) {
        tracing::debug!("from stdin");
        net::stdin(sender)
    } else {
        tracing::debug!("from pipe");
        net::pipein(sender)
    };

    tokio::join!(input_handle, async {
        tokio::select! {
            r = net::tx(sink, proxy) => {
                if let Err(ref e) = r {
                    if e.downcast_ref::<RecvError>() == Some(&RecvError::Closed) {
                        tracing::debug!("pipe end");
                        return Ok(());
                    }
                }
                tracing::debug!("tx end");
                r
            }
            r = net::rx(stream) => {
                tracing::debug!("rx end");
                r
            }
        }
    })
    .1
}
