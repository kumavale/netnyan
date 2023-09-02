mod client;
mod logger;
mod net;
mod pipe;
mod server;

use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    listen: bool,
    #[arg(short = 'p', long = "port")]
    listen_port: Option<u16>,

    destination: Option<String>,
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let args = Args::parse();

    if args.listen {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::debug!("received Ctrl+C");
                std::process::exit(128 + 2);
            },
            r = server::run(args.listen_port) => r?,
        }
        return Ok(());
    }

    if let Some(destination) = args.destination {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                tracing::debug!("received Ctrl+C");
                std::process::exit(128 + 2);
            },
            r = client::run(destination, args.port) => r?,
        }
        return Ok(());
    }

    Args::command().print_help()?;
    std::process::exit(1)
}
