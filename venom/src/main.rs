#[macro_use]
extern crate log;

use std::io;
use std::net::{TcpListener, TcpStream};
use smol::{Async, Task};
use serde::{Serialize, Deserialize};
use crate::link::Link;

mod link;

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxy {
    pub name: String,
    pub listen: u16,
    pub upstream: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub proxies: Vec<Proxy>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error = {0}")]
    Io(io::Error),
    #[error("Configuration error = {0}")]
    Confy(#[from] confy::ConfyError)
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let configuration: Configuration = confy::load_path("./venom.toml")?;
    let mut proxies = Vec::new();

    for proxy in configuration.proxies.into_iter() {
        let p = Task::spawn(async move {
            loop {
                // Listen for incoming connections
                let listen_address = format!("0.0.0.0:{}", proxy.listen);
                let listener = Async::<TcpListener>::bind(listen_address)?;
                info!("Listening on {}", listener.get_ref().local_addr()?);
                let (downstream, peer_addr) = listener.accept().await?;
                info!("Accepted client: {}", peer_addr);

                // Make upstream connection
                let upstream_addr = proxy.upstream.clone();
                let upstream = match Async::<TcpStream>::connect(&upstream_addr).await {
                    Ok(u) => u,
                    Err(e) => {
                        error!("Upstream connection failed. Error = {:?}", e);
                        continue
                    }
                };

                info!("Connected to {:?}", upstream_addr);

                // Start the flow control link
                let mut link = Link::new();
                let o = link.start(downstream, upstream).await;
                error!("Link failed. Error = {:?}", o);
            }

            Ok::<_, io::Error>(())
        });

        proxies.push(p);
    }

    smol::run(async move {
        futures_util::future::join_all(proxies).await;
    });

    Ok(())
}
