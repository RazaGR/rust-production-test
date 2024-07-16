use clap::{value_parser, Arg, ArgMatches, Command};

use crate::settings::Settings;

use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

        println!("TBD: start the webserver on port {}", port);
        start_tokio(port, _settings)?
    }

    Ok(())
}

fn start_tokio(port: u16, _settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let router = Router::new().route("/", get(|| async { "Hello, World!" }));
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, router)
                .with_graceful_shutdown(shutdown_signal())
                .await?;

            println!("shutting down...");

            Ok::<(), anyhow::Error>(())
        })?;

    std::process::exit(0);
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
